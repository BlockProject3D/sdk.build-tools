// Copyright (c) 2024, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::Command;
use serde::Deserialize;
use crate::packager::interface::{Context, Packager};
use crate::packager::util::{CommandExt, ensure_clean_directories, packager_error};

#[derive(Deserialize)]
pub struct Framework {
    name: String,
    identifier: String,
    includes: Option<String>,
    umbrella: Option<String>
}

packager_error! {
    Error {
        Lipo => "failed to run lipo tool",
        InstallNameTool => "failed to run install_name_tool",
        CreateXcFramework => "failed to generate combined XCFramework package"
    }
}

impl Packager for Framework {
    const NAME: &'static str = "Framework";
    type Error = Error;

    fn do_package_target(&self, target: &str, context: &Context) -> Result<(), Self::Error> {
        let bin_dir;
        let res_dir;
        let module_dir;
        let framework_dir = &context.get_target_path(target).join(format!("{}.framework", self.name));
        if target.contains("darwin") {
            bin_dir = format!("{}.framework/Versions/A/", self.name);
            res_dir = format!("{}.framework/Versions/A/Resources", self.name);
            module_dir = format!("{}.framework/Versions/A/Modules", self.name)
        } else {
            bin_dir = format!("{}.framework/", self.name);
            res_dir = format!("{}.framework/", self.name);
            module_dir = format!("{}.framework/Modules", self.name);
        }
        let bin_dir = &context.get_target_path(target).join(bin_dir);
        let res_dir = &context.get_target_path(target).join(res_dir);
        let module_dir = &context.get_target_path(target).join(module_dir);
        ensure_clean_directories([&**framework_dir, &bin_dir, &res_dir, &module_dir])?;
        Command::new("lipo")
            .arg("-create")
            .arg(context.get_bin_path(target))
            .arg("-output")
            .arg(bin_dir.join(&self.name))
            .ensure(Error::Lipo)?;
        Command::new("install_name_tool")
            .arg("-id")
            .arg(format!("@rpath/{}.framework/{}", self.name, self.name))
            .arg(&self.name)
            .current_dir(bin_dir)
            .ensure(Error::InstallNameTool)?;
        if target.contains("darwin") {
            std::os::unix::fs::symlink("A", framework_dir.join("Versions/Current"))?;
            std::os::unix::fs::symlink(format!("Versions/Current/{}", self.name), framework_dir.join(&self.name))?;
            std::os::unix::fs::symlink("Versions/Current/Resources", framework_dir.join("Resources"))?;
            std::os::unix::fs::symlink("Versions/Current/Modules", framework_dir.join("Modules"))?;
        }
        if let Some(includes) = &self.includes {
            if !context.root.join(includes).exists() {
                println!("Warning: Header directory {} not found in crate root!", includes);
            }
            copy_dir::copy_dir(context.root.join(includes), bin_dir.join("Headers"))?;
            if target.contains("darwin") {
                std::os::unix::fs::symlink("Versions/Current/Headers", framework_dir.join("Headers"))?;
            }
            let motherfuckingrust = format!("{}.h", self.name);
            let umbrella = self.umbrella.as_ref().unwrap_or(&motherfuckingrust);
            let umbrella_path = bin_dir.join("Headers").join(umbrella);
            if !umbrella_path.exists() {
                std::fs::write(umbrella_path, "/* Empty generated umbrella header to ensure Xcode can link the framework. */")?;
            }
            std::fs::write(module_dir.join("module.modulemap"), format!("framework module {} {{
    umbrella header \"{}\"

    export *
    module * {{
        export *
    }}
}}

", self.name, umbrella))?;
        }
        let platforms = if target.contains("darwin") {
            "<string>MacOSX</string>"
        } else {
            "<string>iPhoneOS</string>\n        <string>iPadOS</string>"
        };
        let build_number = Command::new("sw_vers").arg("-buildVersion").output_string()?.replace("\n", "");
        let version = context.get_version().split("-").next().unwrap();
        std::fs::write(res_dir.join("Info.plist"), format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>BuildMachineOSBuild</key>
    <string>{}</string>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>{}</string>
    <key>CFBundleIdentifier</key>
    <string>{}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{}</string>
    <key>CFBundlePackageType</key>
    <string>FMWK</string>
    <key>CFBundleShortVersionString</key>
    <string>{}</string>
    <key>CFBundleSupportedPlatforms</key>
    <array>
        {}
    </array>
    <key>CFBundleVersion</key>
    <string>{}</string>
    <key>MinimumOSVersion</key>
    <string>11.0</string>
    <key>UIDeviceFamily</key>
    <array>
        <integer>1</integer>
        <integer>2</integer>
    </array>
</dict>
</plist>
", build_number, self.name, self.identifier, self.name, version, platforms, version))?;
        Ok(())
    }

    fn do_package(&self, context: &Context) -> Result<(), Self::Error> {
        let mut cmd = Command::new("xcrun");
        cmd.arg("xcodebuild").arg("-create-xcframework");
        let framework_dirs: Vec<PathBuf> = context.targets.iter()
            .map(|target| context.get_target_path(target)
                .join(format!("{}.framework", self.name)))
            .collect();
        for dir in &framework_dirs {
            cmd.arg("-framework").arg(dir);
        }
        cmd.arg("-output").arg(context.root.join(format!("target/{}.xcframework", self.name)));
        cmd.ensure(Error::CreateXcFramework)?;
        Ok(())
    }
}
