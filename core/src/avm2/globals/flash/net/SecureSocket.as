package flash.net {
    import flash.security.X509Certificate;
    import flash.security.CertificateStatus;
    import flash.utils.ByteArray;

    import __ruffle__.stub_getter;
    import __ruffle__.stub_method;
    
    public class SecureSocket extends Socket {
        override public native function connect(host:String, port:int):void;

        public function addBinaryChainBuildingCertificate(certificate:ByteArray, trusted:Boolean):void {
            stub_method("flash.net.SecureSocket", "addBinaryChainBuildingCertificate");
        }

        public function get isSupported():Boolean {
            return true;
        }
        

        public function get serverCertificate():X509Certificate {
            stub_getter("flash.net.SecureSocket", "serverCertificate");
            return null;
        }
        
        public function get serverCertificateStatus():String {
            stub_getter("flash.net.SecureSocket", "serverCertificateStatus");
            return CertificateStatus.UNKNOWN;
        }
    }
}