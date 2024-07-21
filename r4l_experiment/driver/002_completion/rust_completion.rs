// SPDX-License-Identifier: GPL-2.0
//! Rust minimal sample.
      

use kernel::prelude::*;
use kernel::sync::Mutex;
use kernel::{bindings,chrdev, file};
use kernel::task::Task;
#[allow(unused)]
use kernel::sync::Arc;
// use core::sync::atomic::{AtomicPtr, Ordering};

const GLOBALMEM_SIZE: usize = 0x1000;

static GLOBALMEM_BUF: Mutex<[u8;GLOBALMEM_SIZE]> = unsafe {
  Mutex::new([0u8;GLOBALMEM_SIZE])
};
#[allow(dead_code)]
static mut COMPLETION_DATA:CompletionData  =  CompletionData::IsEmpty(true);
enum CompletionData {
  IsEmpty(bool),
  Data(bindings::completion),
}

// static mut COMPLETION_DATA: AtomicPtr<CompletionPrvData> =  AtomicPtr::new(core::ptr::null_mut());
// struct CompletionPrvData {
//   completion: bindings::completion,
//   count: usize,
// }

// impl  CompletionPrvData{
//   /// new 
//   fn new() -> Self {
//     // pr_info!("new");
//     let mut _completion = bindings::completion::default();
//     let mut  _self = CompletionPrvData{
//       completion: _completion,
//       count: 100,
//     };
//     unsafe {
//         bindings::init_completion(&mut (_self.completion) as * mut _);
//         bindings::complete(&mut (_self.completion) as *mut _);

//       }
//     pr_info!("new");
//     pr_info!("done {}",_completion.done);
//     _self
//   }
//   // // / get mut ptr
//   // fn get_mut_ptr(&self) -> *mut bindings::completion {
//   //   let ptr = Arc::try_new(self.completion).unwrap();
//   //   Arc::into_raw(ptr) as *mut bindings::completion
//   // } 
//   /// get count
//   fn get_count(&self) -> usize {
//     self.count
//   }
// }
/// init 
pub fn init_completion_data() {
  pr_info!("init_completion_data");

  unsafe {
    COMPLETION_DATA =  CompletionData::Data(bindings::completion::default());
    match COMPLETION_DATA {
      CompletionData::IsEmpty(_) => {},
      CompletionData::Data(ref mut data) => {
      bindings::init_completion(data as * mut _);
      // bindings::complete(data as * mut _);
      }
    }

  }

  // let mut _completion = bindings::completion::default();
  // unsafe {
  //   bindings::init_completion(&mut _completion as *mut bindings::completion);
  //   let completion_ptr = Arc::try_new(_completion).unwrap();
  //   // let raw_ptr = Arc::into_raw(Arc::clone(completion_ptr)) as *mut bindings::completion;
  //   // pr_info!("init_completion_data: ptr is {:?}", raw_ptr);
  //   COMPLETION_DATA =  CompletionData::Data(completion_ptr);

  // }
  // let mut  _test = CompletionPrvData::new();
  // {
  // unsafe {
  //   bindings::complete(& mut (_test.completion) as *mut _);
  //   let mut _com = Arc::try_new(_test).unwrap();
    // pr_info!("after count {}",(*_com).get_count());

    // let  completion = Arc::try_new(_com).unwrap();
    // // let t = &mut CompletionPrvData::new();
    // // pr_info!("count {}",t.get_count());
    // let  raw_ptr = Arc::into_raw(completion) as *mut CompletionPrvData;
    // let ptr = &mut *_com as * mut CompletionPrvData;
    // let  ptr = Arc::into_raw(_com) as *mut CompletionPrvData;
    // pr_info!("count {}",(*ptr).get_count());
    // pr_info!("done {}",(*ptr).completion.done);


    // // bindings::complete(&mut (*ptr).completion as *mut _);

    // // COMPLETION_DATA  = AtomicPtr::new(ptr);
    // COMPLETION_DATA.store( ptr, Ordering::Release);

    // let new_ptr = COMPLETION_DATA.load(Ordering::Acquire);

    // let arc = Arc::from_raw(new_ptr as *mut CompletionPrvData) ;
    // let cloned_arc = Arc::clone(&arc);
    // Arc::into_raw(arc);
    // bindings::complete(&cloned_arc.completion as *const _ as *mut _);
    // let  completion_ptr = COMPLETION_DATA.load(core::sync::atomic::Ordering::Relaxed) as *mut CompletionPrvData;
    // pr_info!("count {}",(*completion_ptr).get_count());
    
    // pr_info!("init_completion_data: ptr is {:?}", completion_ptr);
    // bindings::complete((*completion_ptr).completion as * mut _);
  // }}
  // unsafe {
  //   let  completion_ptr = COMPLETION_DATA.load(core::sync::atomic::Ordering::Relaxed) as *mut CompletionPrvData;
  //   pr_info!("block count {}",(*completion_ptr).get_count());
    
  //   // pr_info!("init_completion_data: ptr is {:?}", completion_ptr);
  //   bindings::complete(&mut (*completion_ptr).completion as * mut _);
  // }
}
// /// get mut ptr
// pub fn get_mut_ptr() -> *mut bindings::completion {
//   pr_info!("get_mut_ptr");
//   unsafe{
//     let  completion_ptr = COMPLETION_DATA.load(core::sync::atomic::Ordering::Relaxed) as *mut CompletionPrvData;
//     if completion_ptr.is_null() {
//       pr_info!("get_mut_ptr: ptr is null");
//     }
//     pr_info!("count {}",(*completion_ptr).get_count());
//     // bindings::complete((*completion_ptr).get_mut_ptr() as * mut _);

//     (*completion_ptr).get_mut_ptr() as * mut bindings::completion

//   }
// }
  
  
//     // (mut_ptr).get_mut_ptr()
// }
/// get count
// pub fn get_count() -> usize {
//   pr_info!("get count");
//   unsafe{
//     let completion_ptr = COMPLETION_DATA.load(core::sync::atomic::Ordering::Relaxed);
//     if completion_ptr.is_null() {
//       pr_info!("get_mut_ptr: ptr is null");
//     }
//     (*completion_ptr).get_count() 

//   }
// }
struct RustFile {
  #[allow(dead_code)]
  inner: &'static Mutex<[u8;GLOBALMEM_SIZE]>,  
}
unsafe impl Send for RustFile {}
unsafe impl Sync for RustFile {}
module! {
  type: RustCompletion,
  name: "rust_completion",
  author: "whocare",
  description: "completion module in rust",
  license: "GPL",
}
      
#[vtable]
impl file::Operations for RustFile {
    type Data = Box<Self>;

    fn open(_shared: &(), _file: &file::File) -> Result<Box<Self>> {
      pr_info!("rust_completion: open");

        Ok(
            Box::try_new(RustFile {
                inner: &GLOBALMEM_BUF,
            })?
        )
    }

    fn write(_this: &Self,_file: &file::File,_reader: &mut impl kernel::io_buffer::IoBufferReader,_offset:u64,) -> Result<usize> {
      pr_info!("write");  
      // pr_info!("rust_completion: count:{}", get_count());
      pr_info!("pid:{}", Task::current().pid());
      unsafe {
        // COMPLETION_DATA =  CompletionData::Data(bindings::completion::default());
        match COMPLETION_DATA {
          CompletionData::IsEmpty(_) => {},
          CompletionData::Data(ref mut data) => {
          // bindings::init_completion(data as * mut _);
          bindings::complete(data as * mut _);
          }
        }
      }
      // unsafe {

      //   let  completion_ptr = COMPLETION_DATA.load(core::sync::atomic::Ordering::Relaxed) as *mut CompletionPrvData;
      //   pr_info!("count {}",(*completion_ptr).get_count());
    
      //   bindings::complete(&mut (*completion_ptr).completion as * mut _);

      // }
      // let ptr = get_mut_ptr();
      // pr_info!("done {}",(*ptr).done);

      // unsafe { bindings::complete(get_mut_ptr() as *mut _) };
      // unsafe {
      //   // let data_ptr = COMPLETION_DATA.load(core::sync::atomic::Ordering::Relaxed);
      //   // pr_info!("rust_completion: count:{} done:{}", (*data_ptr).get_count(),(*data_ptr).completion.done);
      //   // bindings::complete(get_mut_ptr() as *mut _)
      //   // get_count();
      // }
      // pr_info!("rust_completion: write ---pid:{} \n", Task::current().pid());
      // let reader_len = _reader.len();
      pr_info!("data len:{}", _reader.len());
      // pr_info!("writer offset:{}", _offset);
      // Ok(reader_len)
      if _reader.is_empty() {
            Ok(0)
        }else if _offset as usize > GLOBALMEM_SIZE {
            Ok(0)

        } else{
            let reader_len = _reader.len();
            let mut buf = _this.inner.lock();
            _reader.read_slice(&mut buf[..reader_len])?;
            Ok(reader_len)
        }
    }

    fn read(_this: &Self,_file: &file::File,_writer: &mut impl kernel::io_buffer::IoBufferWriter,_offset:u64,) -> Result<usize> {
      pr_info!("read\n");    
      pr_info!("pid:{}", Task::current().pid());
	    pr_info!("process {} is going to sleep\n", Task::current().pid());
      unsafe {
        // COMPLETION_DATA =  CompletionData::Data(bindings::completion::default());
        match COMPLETION_DATA {
          CompletionData::IsEmpty(_) => {},
          CompletionData::Data(ref mut data) => {
            pr_info!("should wait for completion");
          // bindings::init_completion(data as * mut _);
          bindings::wait_for_completion(data as * mut _);
          }
        }
      }
      // get_mut_ptr();
      // let ptr = get_mut_ptr();
      // pr_info!("done {}",(*ptr).done);
      // unsafe {
      //   match  &COMPLETION_DATA {
      //     CompletionData::Empty(_) => {},
      //     CompletionData::Data(data) => {
      //       // pr_info!("rust_completion: data {:?}",data);
      //       let _t  = Arc::clone(&data);
      //       pr_info!("rust_completion: done:{}", (data).done);
      //       let   ptr = Arc::into_raw(_t);
      //       // pr_info!("rust_completion: ptr:{:?}", &ptr);
      //       bindings::complete(ptr as *mut _);
      //     }
      //   }
        // let data_ptr = COMPLETION_DATA.load(core::sync::atomic::Ordering::Relaxed);
        // pr_info!("rust_completion: count:{} done:{}", (*data_ptr).get_count(),(*data_ptr).completion.done);
        // bindings::wait_for_completion(get_mut_ptr() as *mut _)
        // bindings::complete(get_mut_ptr() as *mut _)

        // get_count();
      // }
      // unsafe {

      //   let  completion_ptr = COMPLETION_DATA.load(core::sync::atomic::Ordering::Relaxed) as *mut CompletionPrvData;
      //   pr_info!("count {}",(*completion_ptr).get_count());
    
      //   // pr_info!("init_completion_data: ptr is {:?}", completion_ptr);
      //   bindings::wait_for_completion(&mut (*completion_ptr).completion as * mut _);
      //   // let data_ptr = COMPLETION_DATA.load(core::sync::atomic::Ordering::Relaxed);
      //   // pr_info!("rust_completion: count:{} done:{}", (*data_ptr).get_count(),(*data_ptr).completion.done);
      //   // bindings::wait_for_completion(get_mut_ptr() as *mut _)
      //   // bindings::complete(get_mut_ptr() as *mut _)

      //   // get_count();
      // }
      // pr_info!("rust_completion: count:{}", get_count());
      // unsafe { bindings::wait_for_completion(get_mut_ptr() as *mut _) };
      pr_info!("process {} is waking up\n", Task::current().pid());

      if _offset as usize >= GLOBALMEM_SIZE {
            Ok(0)
        }else {
            let buf = _this.inner.lock();
            // pr_info!("len is {}",_writer.len());
            // pr_info!("offset is {}",_offset);

            _writer.write_slice(&buf[_offset as usize..])?;
            Ok(_writer.len())
        }
        
    }
}

struct RustCompletion {
    _dev: Pin<Box<chrdev::Registration<1>>>,
}
      
impl kernel::Module for RustCompletion {
  fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
    pr_info!("Rust character device sample (init)\n");

    let mut chrdev_reg = chrdev::Registration::new_pinned(_name, 0, _module)?;

    // Register the same kind of device twice, we're just demonstrating
    // that you can use multiple minors. There are two minors in this case
    // because its type is `chrdev::Registration<2>`
    // chrdev_reg.as_mut().register::<RustFile>()?;
    chrdev_reg.as_mut().register::<RustFile>()?;
    init_completion_data();
    Ok(RustCompletion { _dev: chrdev_reg })
  }
}

impl Drop for RustCompletion {
  fn drop(&mut self) {
      pr_info!("Rust character device sample (exit)\n");
  }
}
