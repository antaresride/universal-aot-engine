use std::alloc::{Layout, alloc, dealloc};

// Link to AOT-generated machine code - now unsafe extern
unsafe extern "C" {
    fn rt_get_tag(ptr: *const u8) -> i32;
    fn rt_get_payload(ptr: *const u8, offset: i32) -> i32;
}

/// Universal Enum with reference-counted GC
struct UniversalEnum {
    ptr: *mut u8,
    name: &'static str,
    layout: Layout,
}

impl UniversalEnum {
    /// Allocate: [RC: u64][Tag: u32][Payload...]
    fn new(name: &'static str, tag: u8, data: &[i32]) -> Self {
        let data_size = data.len() * 4;
        let total_size = 8 + 4 + data_size;
        let layout = Layout::from_size_align(total_size, 8).unwrap();

        unsafe {
            let raw_ptr = alloc(layout);
            *(raw_ptr as *mut u64) = 1; // RC = 1

            let data_ptr = raw_ptr.add(8);
            *(data_ptr as *mut u32) = tag as u32; // 4-byte tag

            // Copy payload at offset 4 from data_ptr
            let payload_ptr = data_ptr.add(4) as *mut i32;
            std::ptr::copy_nonoverlapping(data.as_ptr(), payload_ptr, data.len());

            Self {
                ptr: data_ptr,
                name,
                layout,
            }
        }
    }

    fn tag(&self) -> i32 {
        unsafe { rt_get_tag(self.ptr) }
    }

    fn payload(&self, index: usize) -> i32 {
        unsafe { rt_get_payload(self.ptr, (index * 4) as i32) }
    }
}

impl Drop for UniversalEnum {
    fn drop(&mut self) {
        unsafe {
            let raw_ptr = self.ptr.sub(8);
            let count = *(raw_ptr as *mut u64);

            if count == 1 {
                dealloc(raw_ptr, self.layout);
                println!("\x1b[31m[GC]\x1b[0m {} freed", self.name);
            } else {
                *(raw_ptr as *mut u64) = count - 1;
            }
        }
    }
}

fn main() {
    println!("\x1b[1m--- Universal Enum Engine ---\x1b[0m\n");

    {
        // Payment::Card(9999)
        let p = UniversalEnum::new("Payment::Card", 0, &[9999]);

        // Color::Custom(255, 128, 0)
        let c = UniversalEnum::new("Color::Custom", 2, &[255, 128, 0]);

        println!(
            "\x1b[32m[AOT]\x1b[0m {} | Tag: {} | Value: {}",
            p.name,
            p.tag(),
            p.payload(0)
        );
        println!(
            "\x1b[32m[AOT]\x1b[0m {} | Tag: {} | RGB: {}, {}, {}",
            c.name,
            c.tag(),
            c.payload(0),
            c.payload(1),
            c.payload(2)
        );

        println!("\n--- Scope Closing ---");
    }

    println!("\n\x1b[34mExecution Finished.\x1b[0m");
}
