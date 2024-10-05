# Install Unreal Engine 5 on Ubuntu
To install Unreal Engine 5 on Ubuntu, follow these steps:

## 1. Download the Unreal Engine 5 installer:
- Go to the Unreal Engine website (www.unrealengine.com) and sign in with your Epic Games account.
- Click on the “Downloads” tab and select “Linux” as the platform.
- Choose the desired version (e.g., UE 5.0.3) and click on the “Download” button.
- The installer will be downloaded as a .zip file.

## 2. Extract the installer:
- Right-click on the downloaded .zip file and select “Extract Here” or use a file archiver like 7-Zip to extract the contents.
- This will create a new folder containing the Unreal Engine 5 installer.

## 3. Run the installer:
- Open a terminal and navigate to the extracted folder.
- Run the installer by executing the UnrealEngine-Setup.sh script (for Ubuntu 22.04) or UnrealEngine-Setup (for older Ubuntu versions).
- Follow the on-screen instructions to accept the EULA and choose the installation location.

## 4. Configure the installer:
- If you encounter issues with the installer using its own DotNet3, you may need to set UE_USE_SYSTEM_DOTNET=1 as an environment variable before running the installer.
- Additionally, you may need to edit the SetupDotnet.sh script to use the system DotNet6 instead of its own DotNet3 (as mentioned in one of the search results).

## 5. Launch Unreal Engine 5:
- After installation, navigate to the Engine/Binaries/Linux folder and run the UnrealEditor executable.
- This will launch the Unreal Engine 5 editor.

Note: As mentioned in one of the search results, Unreal Engine 5’s main supported Linux distribution is Ubuntu 22.04, although it may run on other distributions like Fedora 36 as well.

Troubleshooting: * If you encounter issues during installation or runtime, refer to the Unreal Engine documentation or community forums for troubleshooting guidance. * Make sure to check the system requirements for Unreal Engine 5 and ensure your Ubuntu system meets the minimum requirements.

By following these steps, you should be able to successfully install Unreal Engine 5 on your Ubuntu system.

