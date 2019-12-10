//
// Created by Camilo Parra on 2019-11-21.
//

#ifndef ARIES_KMS_STORAGE_H
#define ARIES_KMS_STORAGE_H


struct ExternError {
    int32_t code;
    char* message;
};

struct ByteBuffer {
    int64_t len;
    uint8_t *data;
};


/* opens the desired storage by using keyring, enclave, etc
*
*  @param [in] `config` - a json string that details what or where key is stored, type of storage, name of storage
*    {
*    "name": "cam's database",
*    "type": "postgres, couch, etc",
*    "credentials": ["url", "username", password], (should eventually come from lox)
*  }
*  @para
*  @param [out] `error` - an error structure that can be checked if an error occurs. Includes a code and string message. If success, no further checking is needed
*  @return 1 - success, 0 otherwise
*
*/
extern int32_t pyxis_open(const char * const config, const struct ExternError* error);


#endif //ARIES_KMS_STORAGE_H
