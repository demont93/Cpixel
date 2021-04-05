use jni::JNIEnv;
use jni::objects::{JClass};
use jni::sys::{jstring, jlong, jbyteArray, jint};

use crate::converter::{Converter};
use crate::dimensions::Dimensions;
use crate::buffer_2d::Buffer2d;

#[no_mangle]
pub unsafe extern
fn Java_com_demont93_camera_1x_1app_RustBindings_newConverter(
    _: JNIEnv,
    _: JClass,
    output_height: jint,
    output_width: jint,
    input_height: jint,
    input_width: jint,
    resize_type: jint,
) -> jlong {
    Box::into_raw(Box::new(Converter::new(
        Dimensions {
            height: output_height as usize,
            width: output_width as usize,
        },
        Dimensions {
            height: input_height as usize,
            width: input_width as usize,
        },
        true,
        resize_type.into(),
    ))) as i64
}

#[no_mangle]
pub unsafe extern
fn Java_com_demont93_camera_1x_1app_RustBindings_convert(
    env: JNIEnv,
    _: JClass,
    converter_i64: jlong,
    buffer: jbyteArray,
) -> jstring {
    let converter = converter_i64 as *mut Converter;
    let dims = (*converter).image_settings();
    let buffer = env.convert_byte_array(buffer).unwrap();
    let buffer = Buffer2d::new(
        *dims,
        buffer,
    );
    let result = (*converter).convert_one(&buffer);

    let final_string = result.buffer
        .chunks(result.dimensions.width)
        .map(|chunk| {
            chunk.iter().map(|m| m.0).collect::<String>()
        })
        .collect::<Vec<String>>()
        .as_slice()
        .join("\n");
    env.new_string(final_string).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern
fn Java_com_demont93_camera_1x_1app_RustBindings_dropConverter(
    _: JNIEnv,
    _: JClass,
    converter_i64: jlong,
) {
    Box::from_raw(converter_i64 as *mut Converter);
}

#[no_mangle]
pub unsafe extern
fn Java_com_demont93_camera_1x_1app_RustBindings_getOutputWidth(
    _: JNIEnv,
    _: JClass,
    converter_i64: jlong,
) -> jint {
    let converter = Box::from_raw(converter_i64 as *mut Converter);
    let output_dimensions = converter.output_dimensions();
    let width = output_dimensions.width as i32;
    Box::into_raw(converter);
    width
}

#[no_mangle]
pub unsafe extern
fn Java_com_demont93_camera_1x_1app_RustBindings_getOutputHeight(
    _: JNIEnv,
    _: JClass,
    converter_i64: jlong,
) -> jint {
    let converter = Box::from_raw(converter_i64 as *mut Converter);
    let output_dimensions = converter.output_dimensions();
    let height = output_dimensions.height as i32;
    Box::into_raw(converter);
    height
}