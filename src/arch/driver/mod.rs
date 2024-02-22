use super::qemu_opensbi;

pub fn write_to_console_byte(byte : u8) {
    qemu_opensbi::sbi::sbi_debug_console_write_byte(byte);
}