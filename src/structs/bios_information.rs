use super::*;

pub struct SMBiosInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosInformation<'a> {
    const STRUCT_TYPE: u8 = 0u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosInformation<'a> {
    fn vendor(&self) -> Option<String> {
        self.parts.get_field_string(0x4)
    }

    fn version(&self) -> Option<String> {
        self.parts.get_field_string(0x5)
    }

    fn starting_address_segment(&self) -> Option<u16> {
        self.parts.get_field_word(0x6)
    }

    fn release_date(&self) -> Option<String> {
        self.parts.get_field_string(0x8)
    }

    fn rom_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x9)
    }

    fn characteristics(&self) -> Option<u32> {
        self.parts.get_field_dword(0xA)
    }

    fn bios_vendor_reserved_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0xE)
    }

    fn system_vendor_reserved_characteristics(&self) -> Option<u16> {
        self.parts.get_field_word(0x10)
    }

    fn characteristics_extension0(&self) -> Option<u8> {
        self.parts.get_field_byte(0x12)
    }

    fn characteristics_extension1(&self) -> Option<u8> {
        self.parts.get_field_byte(0x13)
    }

    fn system_bios_major_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x14)
    }

    fn system_bios_minor_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x15)
    }

    fn e_c_firmware_major_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x16)
    }

    fn e_c_firmware_minor_release(&self) -> Option<u8> {
        self.parts.get_field_byte(0x17)
    }

    fn extended_rom_size(&self) -> Option<u8> {
        self.parts.get_field_byte(0x18)
    }
}

impl fmt::Debug for SMBiosInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosInformation>())
            .field("header", &self.parts.header)
            .field("vendor", &self.vendor())
            .field("version", &self.version())
            .field("starting_address_segment", &self.starting_address_segment())
            .field("release_date", &self.release_date())
            .field("rom_size", &self.rom_size())
            .field("characteristics", &self.characteristics())
            .field(
                "bios_vendor_reserved_characteristics",
                &self.bios_vendor_reserved_characteristics(),
            )
            .field(
                "system_vendor_reserved_characteristics",
                &self.system_vendor_reserved_characteristics(),
            )
            .field(
                "characteristics_extension0",
                &self.characteristics_extension0(),
            )
            .field(
                "characteristics_extension1",
                &self.characteristics_extension1(),
            )
            .field(
                "system_bios_major_release",
                &self.system_bios_major_release(),
            )
            .field(
                "system_bios_minor_release",
                &self.system_bios_minor_release(),
            )
            .field(
                "e_c_firmware_major_release",
                &self.e_c_firmware_major_release(),
            )
            .field(
                "e_c_firmware_minor_release",
                &self.e_c_firmware_minor_release(),
            )
            .field("extended_rom_size", &self.extended_rom_size())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bios_information() {
        let bios_information_bytes = vec![
            // struct_type(0), length(0x1A), handle(0x0D)
            0x00, 0x1A, 0x0D, 0x00,
            // vendor: Some("Microsoft Corporation"), version: Some("1.2547.140"), starting_address_segment: Some(0), release_date: Some("09/14/2020"),
            // rom_size: Some(255), characteristics: Some(202971264), bios_vendor_reserved_characteristics: Some(0),
            // system_vendor_reserved_characteristics: Some(0), characteristics_extension0: Some(3), characteristics_extension1: Some(13),
            // system_bios_major_release: Some(255), system_bios_minor_release: Some(255), e_c_firmware_major_release: Some(255),
            // e_c_firmware_minor_release: Some(255), extended_rom_size: Some(16) })
            0x01, 0x02, 0x00, 0x00, 0x03, 0xFF, 0x80, 0x18, 0x19, 0x0C, 0x00, 0x00, 0x00, 0x00,
            0x03, 0x0D, 0xFF, 0xFF, 0xFF, 0xFF, 0x10, 0x00,
            // "Microsoft Corporation" (1)
            0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x20, 0x43, 0x6F, 0x72, 0x70,
            0x6F, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x00, // "1.2547.140" (2)
            0x31, 0x2E, 0x32, 0x35, 0x34, 0x37, 0x2E, 0x31, 0x34, 0x30, 0x00,
            // "09/14/2020" (3)
            0x30, 0x39, 0x2F, 0x31, 0x34, 0x2F, 0x32, 0x30, 0x32, 0x30, 0x00,
            // end of structure
            0x00,
        ];

        let parts = SMBiosStructParts::new(bios_information_bytes.as_slice());
        let bios_information = SMBiosInformation::new(&parts);

        // header tests
        assert_eq!(*bios_information.parts().header.handle(), 0x000D);
        assert_eq!(bios_information.parts().header.length(), 0x1A);

        // basic field tests
        assert_eq!(
            bios_information.vendor().expect("vendor field exists"),
            "Microsoft Corporation".to_string()
        );
        assert_eq!(
            bios_information.version().expect("version field exists"),
            "1.2547.140".to_string()
        );
        assert_eq!(
            bios_information
                .starting_address_segment()
                .expect("starting_address_segment field exists"),
            0
        );
        assert_eq!(
            bios_information
                .release_date()
                .expect("release_date field exists"),
            "09/14/2020".to_string()
        );
        assert_eq!(
            bios_information.rom_size().expect("rom_size field exists"),
            0xFF
        );
        assert_eq!(
            bios_information
                .characteristics()
                .expect("characteristics field exists"),
            202971264
        );
        assert_eq!(
            bios_information
                .bios_vendor_reserved_characteristics()
                .expect("bios_vendor_reserved_characteristics field exists"),
            0
        );
        assert_eq!(
            bios_information
                .system_vendor_reserved_characteristics()
                .expect("system_vendor_reserved_characteristics field exists"),
            0
        );
        assert_eq!(
            bios_information
                .characteristics_extension0()
                .expect("characteristics_extension0 field exists"),
            3
        );
        assert_eq!(
            bios_information
                .characteristics_extension1()
                .expect("characteristics_extension1 field exists"),
            13
        );
        assert_eq!(
            bios_information
                .system_bios_major_release()
                .expect("system_bios_major_release field exists"),
            255
        );
        assert_eq!(
            bios_information
                .system_bios_minor_release()
                .expect("system_bios_minor_release field exists"),
            255
        );
        assert_eq!(
            bios_information
                .e_c_firmware_major_release()
                .expect("e_c_firmware_major_release field exists"),
            255
        );
        assert_eq!(
            bios_information
                .e_c_firmware_minor_release()
                .expect("e_c_firmware_minor_release field exists"),
            255
        );
        assert_eq!(
            bios_information
                .extended_rom_size()
                .expect("extended_rom_size field exists"),
            16
        );

        // debug print test
        println!("bios_information: {:?}", bios_information);
    }
}