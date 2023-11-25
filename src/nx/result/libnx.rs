use crate::nx::result::NxResult;

/* For interopability, we'll just steal libnx's results. */

const MODULE: u32 = 345;

mod_result!(BAD_RELOC, 1);
mod_result!(OUT_OF_MEMORY, 2);
mod_result!(ALREADY_MAPPED, 3);
mod_result!(BAD_GET_INFO_STACK, 4);
mod_result!(BAD_GET_INFO_HEAP, 5);
mod_result!(BAD_QUERY_MEMORY, 6);
mod_result!(ALREADY_INITIALIZED, 7);
mod_result!(NOT_INITIALIZED, 8);
mod_result!(NOT_FOUND, 9);
mod_result!(IO_ERROR, 10);
mod_result!(BAD_INPUT, 11);
mod_result!(BAD_REENT, 12);
mod_result!(BUFFER_PRODUCER_ERROR, 13);
mod_result!(HANDLE_TOO_EARLY, 14);
mod_result!(HEAP_ALLOC_FAILED, 15);
mod_result!(TOO_MANY_OVERRIDES, 16);
mod_result!(PARCEL_ERROR, 17);
mod_result!(BAD_GFX_INIT, 18);
mod_result!(BAD_GFX_EVENT_WAIT, 19);
mod_result!(BAD_GFX_QUEUE_BUFFER, 20);
mod_result!(BAD_GFX_DEQUEUE_BUFFER, 21);
mod_result!(APPLET_CMDID_NOT_FOUND, 22);
mod_result!(BAD_APPLET_RECEIVE_MESSAGE, 23);
mod_result!(BAD_APPLET_NOTIFY_RUNNING, 24);
mod_result!(BAD_APPLET_GET_CURRENT_FOCUS_STATE, 25);
mod_result!(BAD_APPLET_GET_OPERATION_MODE, 26);
mod_result!(BAD_APPLET_GET_PERFORMANCE_MODE, 27);
mod_result!(BAD_USB_COMMS_READ, 28);
mod_result!(BAD_USB_COMMS_WRITE, 29);
mod_result!(INIT_FAIL_SM, 30);
mod_result!(INIT_FAIL_AM, 31);
mod_result!(INIT_FAIL_HID, 32);
mod_result!(INIT_FAIL_FS, 33);
mod_result!(BAD_GET_INFO_RNG, 34);
mod_result!(JIT_UNAVAILABLE, 35);
mod_result!(WEIRD_KERNEL, 36);
mod_result!(INCOMPAT_SYS_VER, 37);
mod_result!(INIT_FAIL_TIME, 38);
mod_result!(TOO_MANY_DEV_OP_TABS, 39);
mod_result!(DOMAIN_MESSAGE_UNKNOWN_TYPE, 40);
mod_result!(DOMAIN_MESSAGE_TOO_MANY_OBJECT_IDS, 41);
mod_result!(APPLET_FAILED_TO_INITIALIZE, 42);
mod_result!(APM_FAILED_TO_INITIALIZE, 43);
mod_result!(NVINFO_FAILED_TO_INITIALIZE, 44);
mod_result!(NVBUF_FAILED_TO_INITIALIZE, 45);
mod_result!(LIB_APPLET_BAD_EXIT, 46);
mod_result!(INVALID_CMIF_OUT_HEADER, 47);
mod_result!(SHOULD_NOT_HAPPEN, 48);
