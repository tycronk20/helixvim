//
//  MMHelixBridge.m
//  MacVim
//
//  Bridge between MacVim's Objective-C code and Helix's Rust code.
//

#import "MMHelixBridge.h"

// Import the C functions from the Rust bridge
extern const char* machelix_version(void);
extern bool machelix_init(void);
extern bool machelix_process_key(const char* key);
extern void machelix_free_string(char* s);

@implementation MMHelixBridge

+ (instancetype)sharedInstance {
    static MMHelixBridge *instance = nil;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        instance = [[self alloc] init];
    });
    return instance;
}

+ (NSString *)version {
    const char *version = machelix_version();
    NSString *versionString = [NSString stringWithUTF8String:version];
    machelix_free_string((char *)version);
    return versionString;
}

+ (BOOL)initializeHelix {
    return machelix_init();
}

+ (BOOL)processKey:(NSString *)key {
    const char *keyStr = [key UTF8String];
    return machelix_process_key(keyStr);
}

@end