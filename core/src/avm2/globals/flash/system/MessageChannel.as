package flash.system {
    import flash.events.EventDispatcher;
    import __ruffle__.stub_constructor;
    import __ruffle__.stub_method;
    import __ruffle__.stub_getter;
    
    [API("682")]
    public final class MessageChannel extends EventDispatcher {
        private var _state:String = MessageChannelState.OPEN;

        public function MessageChannel() {
            super();
            stub_constructor("flash.system.MessageChannel");
        }

        public function get state():String {
            stub_method("flash.system.MessageChannel", "state");
            return _state;
        }

        public function get messageAvailable():Boolean {
            stub_getter("flash.system.MessageChannel", "messageAvailable");
            return false;
        }

        public function send(arg:*, queueLimit:int = -1):void {
            stub_method("flash.system.MessageChannel", "send");
        }

        public function close():void {
            stub_method("flash.system.MessageChannel", "close");
            _state = MessageChannelState.CLOSED;
        }

        public function receive(blockUntilReceived:Boolean = false):* {
            stub_method("flash.system.MessageChannel", "receive");
            return "";
        }
    }
}
