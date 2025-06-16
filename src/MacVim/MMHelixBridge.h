//
//  MMHelixBridge.h
//  MacVim
//
//  Bridge between MacVim's Objective-C code and Helix's Rust code.
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface MMHelixBridge : NSObject

/// Returns the MacHelix version string
+ (NSString *)version;

/// Initialize the Helix editor
+ (BOOL)initializeHelix;

/// Process a keystroke
+ (BOOL)processKey:(NSString *)key;

/// Singleton instance
+ (instancetype)sharedInstance;

@end

NS_ASSUME_NONNULL_END