

__all__ = ['fingerprint_analyzer']

class FingerprintAnalyzer:
    def char_shift(self, c, shift, base):
        c_u8 = ord(c)
        return chr(((c_u8 - base + shift) % 26) + base)


    def decrypt_vigenere(self, encrypted_data, secret):
        cleaned_secret = ''.join([c for c in secret if c.isalpha()])
        if not cleaned_secret:
            cleaned_secret = 'XGGHRDFGJKWKLGNG'
        else:
            cleaned_secret = cleaned_secret.upper()

        output = []
        secret_bytes = [26 - (ord(c) - ord('A')) for c in cleaned_secret]

        for i, c in enumerate(encrypted_data):
            base = ord('a') if c.islower() else ord('A')

            if c.isalpha():
                output.append(self.char_shift(c, secret_bytes[i % len(secret_bytes)], base))
            else:
                output.append(c)

        return ''.join(output)

    def calculate_checksum(self, data):
        return sum(ord(c) for c in data)


    def decrypt(self, encrypted_data_with_checksum, secret):
        if len(encrypted_data_with_checksum) < 9:
            raise ValueError("Not enough data")

        encrypted_data = encrypted_data_with_checksum[:-8]
        checksum_str = encrypted_data_with_checksum[-8:]
        checksum = int(checksum_str, 16)

        decrypted_data = self.decrypt_vigenere(encrypted_data, secret)

        calculated_checksum = self.calculate_checksum(encrypted_data)
        if calculated_checksum == checksum:
            return decrypted_data
        else:
            raise ValueError("Mismatch")
        
fingerprint_analyzer = FingerprintAnalyzer()