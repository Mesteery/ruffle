package flash.system
{
    import flash.events.EventDispatcher;
    import __ruffle__.stub_constructor;
    import __ruffle__.stub_method;

    [API("682")]
    public class Worker extends EventDispatcher
    {
        private static var _current:Worker;

        public function Worker()
        {
            super();
            stub_constructor("flash.system.Worker");
        }

        public static function get current():Worker
        {
            stub_method("flash.system.Worker", "current");
            if (!_current)
            {
                _current = new Worker();
            }
            return _current;
        }

        public static function get isSupported():Boolean
        {
            return false;
        }

        public function get isPrimordial():Worker
        {
            stub_method("flash.system.Worker", "isPrimordial");
            return true;
        }

        public function start():void
        {
            stub_method("flash.system.Worker", "start");
        }

        public function setSharedProperty(key:String, value:*):void
        {
            stub_method("flash.system.Worker", "setSharedProperty");
        }

        public function getSharedProperty(key:String):*
        {
            stub_method("flash.system.Worker", "getSharedProperty");
            return null;
        }

        public function createMessageChannel(receiver:Worker):MessageChannel
        {
            stub_method("flash.system.Worker", "createMessageChannel");
            return new MessageChannel();
        }

        public function terminate():void
        {
            stub_method("flash.system.Worker", "terminate");
        }
    }
}
