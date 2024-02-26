from fingerprint_analyze import fingerprint_analyzer


if __name__ == "__main__":
    content = "U2FsdGsafasasfasfadsfadsVkX1"
    secret = "XGGHRDFGJKWKLGNG"
    try:
        session = fingerprint_analyzer.decrypt(content, secret)
        print(session)
    except ValueError as e:
        print(e)
        
    content1 = "I48O36G2000001E6"
    secret1 = "hello"
    try:
        session = fingerprint_analyzer.decrypt(content1, secret1)
        print(session)
    except ValueError as e:
        print(e)
