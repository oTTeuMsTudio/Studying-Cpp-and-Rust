# Install UE 5.4.4 on Ubuntu 24.04

Here’s a step-by-step guide to install Unreal Engine 5.4.4 on Ubuntu 24.04:

## 1. Download Unreal Engine 5.4.4:
- Visit the official Unreal Engine website (www.unrealengine.com) and click on the “Download Unreal Engine” button.
- Select the “Linux” option and choose the “Ubuntu 22.04” package (as Unreal Engine 5.4.4 is compatible with Ubuntu 22.04 and later).
- Download the .zip file containing the Unreal Engine 5.4.4 installation package.

## 2. Extract the installation package:
- Extract the downloaded .zip file to a directory of your choice (e.g., ~/UnrealEngine).

## 3. Run the installer:
- Navigate to the extracted directory and run the SetupUE.sh script (or SetupUE.bat on Windows) as a terminal command.
- Follow the installer prompts to select the installation components and directory.

## 4. Configure the installation:
- Set UE_USE_SYSTEM_DOTNET=1 as an environment variable to use the system-installed .NET 6 instead of the bundled .NET 3.
- Edit the SetupDotnet.sh script to read the host .NET version correctly (as described in one of the search results).

## 5. Launch Unreal Engine:
- Run the UnrealEditor executable from the Engine/Binaries/Linux directory.
- You can also use the Epic Asset Manager to download and manage your projects and assets.

### Additional Notes:

Make sure your system meets the minimum and recommended hardware specifications for Unreal Engine 5.4.4, as listed on the official Unreal Engine website.
If you encounter any issues during installation, refer to the Unreal Engine documentation or seek help from the Epic Games community forums.
By following these steps, you should be able to successfully install Unreal Engine 5.4.4 on your Ubuntu 24.04 system.
