use embedded_storage::ReadStorage;

pub fn hash(flash: &mut impl ReadStorage) -> u32 {
	let mut state: u32 = 0;
	
	let size: u32 = flash.capacity() as u32; // number of bytes

	// read 4 bytes at a time
	// while loops

	let mut currentsize: u32 = 0;
	let mut buf = [0; 4];

	while currentsize < size {
		if flash.read(currentsize, &mut buf).is_err() {
			panic!("coudn't read flash");
		}
		
		state ^= u32::from_le_bytes(buf);
		
		currentsize += 4;
	}
	state
}