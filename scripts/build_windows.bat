@echo off
rem Build Rust core
cargo build --release --manifest-path core/Cargo.toml

rem Build Go dashboard
cd dashboard
go build -o ../dist/dashboard.exe
cd ..

rem Package Python dependencies (not installing—just copying scripts)
mkdir -p dist/python
xcopy /y analytics\*.py dist\python\

rem Create a packaged folder
mkdir dist\mevbot_package
xcopy /y target\release\core.exe dist\mevbot_package\core.exe
xcopy /y dist\dashboard.exe dist\mevbot_package\dashboard.exe
xcopy /y scripts\run_mevbot.bat dist\mevbot_package\

rem Zip
powershell Compress-Archive -Path dist\mevbot_package\* -DestinationPath dist\mevbot_package.zip
echo Packaged at dist\mevbot_package.zip
