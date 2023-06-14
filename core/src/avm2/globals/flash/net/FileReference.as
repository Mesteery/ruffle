package flash.net
{
    import flash.events.EventDispatcher;
    import flash.utils.ByteArray;
    import __ruffle__.stub_method;

    public class FileReference extends EventDispatcher
    {
        public function FileReference() {
            
        }

        public function get creationDate(): Date {
            stub_method("flash.net.FileReference", "creationDate");
            return new Date();
        }   

        public function get creator(): String {
            stub_method("flash.net.FileReference", "creator");
            return "";
        }   

        public function get data(): ByteArray {
            stub_method("flash.net.FileReference", "data");
            return new ByteArray();
        }   

        public function get extension(): String {
            stub_method("flash.net.FileReference", "extension");
            return "";
        }   

        public function get modificationDate(): Date {
            stub_method("flash.net.FileReference", "modificationDate");
            return new Date();
        }   

        public function get name(): String {
            stub_method("flash.net.FileReference", "name");
            return "";
        }   

        public static function get permissionStatus(): String {
            stub_method("flash.net.FileReference", "permissionStatus");
            return "granted";
        }   

        public function get size(): Number {
            stub_method("flash.net.FileReference", "size");
            return 0;
        }   

        public function get type(): String {
            stub_method("flash.net.FileReference", "type");
            return "";
        }   

        public function browse(typeFilter:Array = null):Boolean {
            stub_method("flash.net.FileReference", "browse");
            return false;
        }   

        public function cancel():void {
            stub_method("flash.net.FileReference", "cancel");
        }   

        public function download(request:URLRequest, defaultFileName:String = null):void {
            stub_method("flash.net.FileReference", "download");
        }   

        public function load():void {
            stub_method("flash.net.FileReference", "load");
        }   

        public function requestPermission():void {
            stub_method("flash.net.FileReference", "requestPermission");
        }   

        public function save(data:*, defaultFileName:String = null):void {
            stub_method("flash.net.FileReference", "save");
        }   

        public function upload(request:URLRequest, uploadDataFieldName:String = "Filedata", testUpload:Boolean = false):void {
            stub_method("flash.net.FileReference", "upload");
        }   

        public function uploadUnencoded(request:URLRequest):void {
            stub_method("flash.net.FileReference", "uploadUnencoded");
        }
    }
}