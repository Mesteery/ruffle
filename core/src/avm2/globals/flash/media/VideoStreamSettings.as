// The initial version of this file was autogenerated from the official AS3 reference at
// https://help.adobe.com/en_US/FlashPlatform/reference/actionscript/3/flash/media/VideoStreamSettings.html
// by https://github.com/golfinq/ActionScript_Event_Builder
// It won't be regenerated in the future, so feel free to edit and/or fix

package flash.media
{
    import __ruffle__.stub_method;

    [API("674")]
    public class VideoStreamSettings
    {
        // Retrieve the maximum amount of bandwidth that the current outgoing video feed can use, in bytes per second.
        private var _bandwidth: int;

        // Video codec used for compression.
        private var _codec: String;

        // The maximum frame rate at which the video frames are encoded, in frames per second.
        private var _fps: Number;

        // The current encoded height, in pixels.
        private var _height: int;

        // The number of video frames transmitted in full (called keyframes or IDR frames) instead of being interpolated by the video compression algorithm.
        private var _keyFrameInterval: int;

        // The required level of picture quality, as determined by the amount of compression being applied to each video frame.
        private var _quality: int;

        // The current encoded width, in pixels.
        private var _width: int;

        // The number of video frames transmitted in full (called keyframes or Instantaneous Decoding Refresh (IDR) frames) instead of being interpolated by the video compression algorithm.
        public function setKeyFrameInterval(keyFrameInterval:int = 15):void
        {
            stub_method("flash.media.VideoStreamSettings", "setKeyFrameInterval");
        }

        // Sets the resolution and frame rate used for video encoding.
        public function setMode(width:int = -1, height:int = -1, fps:Number = -1):void
        {
            stub_method("flash.media.VideoStreamSettings", "setMode");
        }

        // Sets maximum amount of bandwidth per second or the required picture quality that the current outgoing video feed can use.
        public function setQuality(bandwidth:int = 16384, quality:int = 0):void
        {
            stub_method("flash.media.VideoStreamSettings", "setQuality");
        }

        public function get bandwidth() : int
        {
            return this._bandwidth;
        }

        public function get codec() : String
        {
            return this._codec;
        }

        public function get fps() : Number
        {
            return this._fps;
        }

        public function get height() : int
        {
            return this._height;
        }

        public function get keyFrameInterval() : int
        {
            return this._keyFrameInterval;
        }

        public function get quality() : int
        {
            stub_getter("flash.media.VideoStreamSettings", "quality");
            return this._quality;
        }

        public function get width() : int
        {
            return this._width;
        }
    }
}
