# index file structure
reversed using imhex pattern editor ❤

zstandard

```c
struct IndexHeader{
    char sign[4]; //TGP2
    u32 version; //4
    u32 files_count;
    u32 chunk_size;
    u64 unknown_field0; //0x0
    
    u64 absolute_offset_to_data; //offset to data in near archive file
    u64 content_size; //file_size - absolute_offset_to_data
    
    u64 unknown_field1; //0x0
    u64 unknown_field2;
};

struct PackedTable{
    u32 size_bytes; //0x00000000 :)
    char zstd_data[size_bytes];
};

struct Indexator{
    IndexHeader header;
    
    u32 nametable_size_unpacked; //why only for this table?
    PackedTable name_table;
    PackedTable file_info_table;
    PackedTable c;
};

Indexator index @ 0x00;
```
# tables structures
### nametable (first)
```c
struct nametablefield{
u16 filepath_length;
char filepath[filepath_length+1]; //null terminer
};
```
### fileinfotable (second)
```c
struct FileInfoEntry{
    u64 file_size;
    u64 file_absolute_offset;
    
    u32 first_chunk_index; //data is data devided in blocks. it represent index of first block
    u32 fnv1a_name; //fnv1a hash of string in nametable without \x00 + reverse
    // ^^ example: gamedata/def/ex/car_editor_weapons_ex.lua => hash 76 d2 a6 1e, result 1e a6 d2 76
    
    u64 offset_name; //offset to string in nametable
    
    u32 filename_length;
    u32 unk3; //always 0x1?
    
    u128 md5_hash;
    
    u64 align; //0x0
};

//what is 3rd table? possible data to recover if corrupted archive?
```

# pk2 file
just chunks, encoded in TGE or zstandard.. 
what is TGE? TGE impl in python:
```python
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms
from cryptography.hazmat.backends import default_backend
def chachaDecrypt(chachakey,nonce,data_c) -> bytes:
    cipher = Cipher(algorithms.ChaCha20(chachakey, nonce), mode=None, backend=default_backend())
    decryptor = cipher.decryptor()
    data = decryptor.update(data_c) + decryptor.finalize()
    return data
#hardc in game code key and nonce
key = bytes([0x38, 0x79, 0x2F, 0x42, 0x3F, 0x45, 0x28, 0x48, 0x2B, 0x4D, 0x62, 0x51, 0x65, 0x54, 0x68, 0x57, 0x6D, 0x5A, 0x71, 0x34, 0x74, 0x37, 0x77, 0x39, 0x7A, 0x24, 0x43, 0x26, 0x46, 0x29, 0x4A, 0x40])
nonce = bytes([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2B, 0x4D, 0x62, 0x51, 0x65, 0x54, 0x68, 0x57])
TGE_DATA_WITHOUT_HEADERS_8BYTE = bytes()
BuildResult = chachaDecrypt(key,nonce,TGE_DATA_WITHOUT_HEADERS_8BYTE)
htd = ' '.join(f'{byte:02X}' for byte in BuildResult)
print(f"{htd}")
```
# grp file
