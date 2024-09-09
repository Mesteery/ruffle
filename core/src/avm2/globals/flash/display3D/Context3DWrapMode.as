// The initial version of this file was autogenerated from the official AS3 reference at
// https://help.adobe.com/en_US/FlashPlatform/reference/actionscript/3/flash/display3D/Context3DWrapMode.html
// by https://github.com/golfinq/ActionScript_Event_Builder
// It won't be regenerated in the future, so feel free to edit and/or fix

package flash.display3D
{

    [API("686")]
    public final class Context3DWrapMode
    {
        // Clamp texture coordinates outside the 0..1 range.
        public static const CLAMP:String = "clamp";

        // Clamp in U axis but Repeat in V axis.
        [API("696")] // the docs don't mention it, but this is correct
        public static const CLAMP_U_REPEAT_V:String = "clamp_u_repeat_v";

        // Repeat (tile) texture coordinates outside the 0..1 range.
        public static const REPEAT:String = "repeat";

        // Repeat in U axis but Clamp in V axis.
        [API("696")] // the docs don't mention it, but this is correct
        public static const REPEAT_U_CLAMP_V:String = "repeat_u_clamp_v";

    }
}
