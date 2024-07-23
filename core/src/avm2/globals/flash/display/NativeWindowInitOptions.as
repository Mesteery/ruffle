// The initial version of this file was autogenerated from the official AS3 reference at
// https://help.adobe.com/en_US/FlashPlatform/reference/actionscript/3/flash/display/NativeWindowInitOptions.html
// by https://github.com/golfinq/ActionScript_Event_Builder
// It won't be regenerated in the future, so feel free to edit and/or fix

package flash.display
{
  [API("661")]
  public class NativeWindowInitOptions
  {

    // Specifies whether the window can be maximized by the user.
    public var maximizable:Boolean;

    // Specifies whether the window can be minimized by the user.
    public var minimizable:Boolean;

    // Specifies the NativeWindow object that should own any windows created with this NativeWindowInitOptions.
    [API("671")]
    public var owner:NativeWindow;

    // Specifies the render mode of the NativeWindow object created with this NativeWindowInitOptions.
    [API("675")]
    public var renderMode:String;

    // Specifies whether the window can be resized by the user.
    public var resizable:Boolean;

    // Specifies whether system chrome is provided for the window.
    public var systemChrome:String;

    // Specifies whether the window supports transparency and alpha blending against the desktop.
    public var transparent:Boolean;

    // Specifies the type of the window to be created.
    public var type:String;

    public function NativeWindowInitOptions()
    {
      systemChrome = "standard";
      type = "normal";
      transparent = false;
      owner = null;
      resizable = true;
      maximizable = true;
      minimizable = true;
    }

  }
}
