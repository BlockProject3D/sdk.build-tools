local templates = {}

templates.MODULE_MAP_TEMPLATE = [[
framework module {NAME} {
    umbrella header "{UMBRELLA}"

    export *
    module * {
        export *
    }
}
]]

templates.PLIST = [[
<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">
<plist version=\"1.0\">
<dict>
    <key>BuildMachineOSBuild</key>
    <string>{BUILD_NUMBER}</string>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>{NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>{IDENTIFIER}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{NAME}</string>
    <key>CFBundlePackageType</key>
    <string>FMWK</string>
    <key>CFBundleShortVersionString</key>
    <string>{VERSION}</string>
    <key>CFBundleSupportedPlatforms</key>
    <array>
        {PLATFORMS}
    </array>
    <key>CFBundleVersion</key>
    <string>{VERSION}</string>
    <key>MinimumOSVersion</key>
    <string>11.0</string>
    <key>UIDeviceFamily</key>
    <array>
        <integer>1</integer>
        <integer>2</integer>
    </array>
</dict>
</plist>
]]

return templates
