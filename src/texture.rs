use std::ffi::CString;
use std::ptr::copy_nonoverlapping as memcpy;
use anyhow::Result;
use vulkanalia::prelude::v1_0::*;
use crate::AppData;
use crate::sharedBuffer::create_buffer;
use crate::sharedImages::create_image;
use crate::sharedImages::create_image_view;
use crate::sharedImages::transition_image_layout;
use crate::sharedImages::copy_buffer_to_image;
use stb_image::stb_image;

//================================================
// Texture
//================================================

pub unsafe fn create_texture_image(instance: &Instance, device: &Device, data: &mut AppData) -> Result<()> {
    // Load

    /*
    // version texture png
    let image = File::open("resources/texture.png")?;

    let decoder = png::Decoder::new(image);
    let mut reader = decoder.read_info()?;

    let mut pixels = vec![0; reader.info().raw_bytes()];
    reader.next_frame(&mut pixels)?;

    let size = reader.info().raw_bytes() as u64;
    let (width, height) = reader.info().size();
    */

    // version stb_image
    let mut x: i32 =0;
    let mut y: i32 =0;
    let mut channelsInFile: i32 =0;
    let tgaFileName:std::ffi::CString = CString::new("resources/a_lflift_d02.tga").expect("CString new failed");

    let mut stbPixels = stb_image::stbi_load(     tgaFileName.as_ptr(), 
                                                        &mut x, 
                                                        &mut y, 
                                                        &mut channelsInFile,          // RGB 
                                                        4);         // RGBA

    let size:u64                    = /*tga_imageSize*/ (x * y * 4) as u64;
    let width:u32                   = x as u32;
    let height:u32                  = y as u32;
    let mut pixels:Vec<u8>          = Vec::new();

    let mut iCounter:usize =0;
    loop 
    {
        let CompB = *stbPixels;
        stbPixels = stbPixels.wrapping_add(1);

        pixels.push(CompB);
        iCounter += 1;
        
        if iCounter>= size as usize    {break;};
    }

    /*
    let mut tgaImageFile    = File::open("resources/a_lflift_d02.tga")?;
    
    
    let mut readBuffer = Vec::new();
    tgaImageFile.read_to_end(&mut readBuffer);

    let tga_idLength        : u8    = readBuffer[0];
    let tga_colorMapType    : u8    = readBuffer[1];
    let tga_imageTypeCode   : u8    = readBuffer[2];
    //unsigned char colorMapSpec[5];  // 3 4 5 6 7
    let tga_xOrigin         : u16   = readBuffer[9]  as u16 * 256   + readBuffer[8]  as u16;    // little endian
    let tga_yOrigin         : u16   = readBuffer[11] as u16 * 256   + readBuffer[10] as u16;    // little endian
    let tga_width           : u16   = readBuffer[13] as u16 * 256   + readBuffer[12] as u16;    // little endian
    let tga_height          : u16   = readBuffer[15] as u16 * 256   + readBuffer[14] as u16;    // little endian
    let tga_bpp             : u8    = readBuffer[16];
    let tga_imageDesc       : u8    = readBuffer[17];

    let tga_bytesPerPixel   : u8    = 4 /*3*/ /*tga_bpp / 8*/;  // a priori l'image source a 4 octets per pixel (RGBA)
    let dst_bytesPerPixel   : u8    = 4 /*(tga_bpp/8) + 1*/;   // +1 for A8
    let tga_imageSize       : u32   = tga_width as u32 * tga_height as u32 * tga_bytesPerPixel as u32;
    let dst_imageSize       : u32   = tga_width as u32 * tga_height as u32 * dst_bytesPerPixel as u32;

    if tga_imageTypeCode != 0x02 /*TFT_RGB*/ 
    {
        panic!("Image format not supported");       // TFT_RGB est RGB non compressé
    }

    let mut tga_iCounter:usize      = 18 as usize + tga_idLength as usize;       // skip the id
    let size:u64                    = /*tga_imageSize*/ dst_imageSize as u64;
    let width:u32                   = tga_width as u32;
    let height:u32                  = tga_height as u32;
    let mut pixels:Vec<u8>          = Vec::new();

    let mut iCounter:usize =0;
    loop 
    {
        //Swap the red and blue to make the data RGB instead of BGR
        
        let CompB = readBuffer[iCounter   +tga_iCounter];
        iCounter += 1;
        let CompG = readBuffer[iCounter   +tga_iCounter];
        iCounter += 1;
        let CompR = readBuffer[iCounter   +tga_iCounter];
        iCounter += 1;
        let CompA = readBuffer[iCounter   +tga_iCounter];
        iCounter += 1;
                
        pixels.push(/*0x00*/ CompR);        // R      
        pixels.push(/*0x00*/ CompG);        // G 
        pixels.push(/*0xFF*/ CompB);        // B
        pixels.push(/*0x00*/ CompA);        // A with no effect

        //iCounter += 3;
        
        /*
        let CompB = readBuffer[iCounter+0+tga_iCounter];
        pixels.push(CompB);
        iCounter += 1;
        
        //if iCounter%3==0  {pixels.push(0x00);}
        */   

        if iCounter>= tga_imageSize as usize    {break;};
    }
    */

    // Create (staging)

    let (staging_buffer, staging_buffer_memory) = create_buffer(
        instance,
        device,
        data,
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
    )?;

    // Copy (staging)

    let memory = device.map_memory(staging_buffer_memory, 0, size, vk::MemoryMapFlags::empty())?;

    memcpy(pixels.as_ptr(), memory.cast(), pixels.len());

    device.unmap_memory(staging_buffer_memory);

    // Create (image)

    let (texture_image, texture_image_memory) = create_image(
        instance,
        device,
        data,
        width,
        height,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageTiling::OPTIMAL,
        vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    )?;

    data.texture_image = texture_image;
    data.texture_image_memory = texture_image_memory;

    // Transition + Copy (image)

    transition_image_layout(
        device,
        data,
        data.texture_image,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
    )?;

    copy_buffer_to_image(device, data, staging_buffer, data.texture_image, width, height)?;

    transition_image_layout(
        device,
        data,
        data.texture_image,
        vk::Format::R8G8B8A8_SRGB,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    )?;

    // Cleanup

    device.destroy_buffer(staging_buffer, None);
    device.free_memory(staging_buffer_memory, None);

    Ok(())
}

pub unsafe fn create_texture_image_view(device: &Device, data: &mut AppData) -> Result<()> {
    data.texture_image_view = create_image_view(device, data.texture_image, vk::Format::R8G8B8A8_SRGB)?;

    Ok(())
}

pub unsafe fn create_texture_sampler(device: &Device, data: &mut AppData) -> Result<()> {
    let info = vk::SamplerCreateInfo::builder()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .address_mode_u(vk::SamplerAddressMode::REPEAT)
        .address_mode_v(vk::SamplerAddressMode::REPEAT)
        .address_mode_w(vk::SamplerAddressMode::REPEAT)
        .anisotropy_enable(true)
        .max_anisotropy(16.0)
        .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
        .unnormalized_coordinates(false)
        .compare_enable(false)
        .compare_op(vk::CompareOp::ALWAYS)
        .mipmap_mode(vk::SamplerMipmapMode::LINEAR);

    data.texture_sampler = device.create_sampler(&info, None)?;

    Ok(())
}
