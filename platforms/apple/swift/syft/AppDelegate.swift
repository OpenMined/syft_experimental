//
//  AppDelegate.swift
//  syft
//
//  Created by Madhava Jay on 28/7/20.
//  Copyright © 2020 Madhava Jay. All rights reserved.
//

import UIKit

class Output {
    var bytes: UInt32 = 0
}

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {
    func application(_ application: UIApplication, didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        // Override point for customization after application launch.
        startup()
        return true
    }
    
    func startup() {
        //[::] means all ipv6 interfaces like 0.0.0.0 in ipv4
        start("[::]", 50051)
        
        var output = Output()
        print("Start", output.bytes)
        
        let pointer = UnsafeMutablePointer<Output>.allocate(capacity: 1)
        pointer.initialize(to: output)
        defer {
            pointer.deallocate()
        }
        
        func sum(num: UInt32, ouput: UnsafeMutableRawPointer?) {
            print("sum callback with input: \(num)")
            if let x = ouput?.load(as: Output.self) {
                x.bytes = num
            }
        }

        let handle = CallbackHandle(callback: sum, return_data: pointer)

        register_handler("sum", handle)
        
        print("End", output.bytes)
        
        let timer = Timer.scheduledTimer(withTimeInterval: 3.0, repeats: true) { (timer) in
            // Do what you need to do repeatedly
            print("Output is", output.bytes)
        }
    }
    
    // MARK: UISceneSession Lifecycle

    func application(_ application: UIApplication, configurationForConnecting connectingSceneSession: UISceneSession, options: UIScene.ConnectionOptions) -> UISceneConfiguration {
        // Called when a new scene session is being created.
        // Use this method to select a configuration to create the new scene with.
        return UISceneConfiguration(name: "Default Configuration", sessionRole: connectingSceneSession.role)
    }

    func application(_ application: UIApplication, didDiscardSceneSessions sceneSessions: Set<UISceneSession>) {
        // Called when the user discards a scene session.
        // If any sessions were discarded while the application was not running, this will be called shortly after application:didFinishLaunchingWithOptions.
        // Use this method to release any resources that were specific to the discarded scenes, as they will not return.
    }


}

