package flash.filesystem
{
  import flash.net.FileReference;
  import flash.desktop.Icon;
  import flash.errors.*;
  import flash.system.Capabilities;
  import __ruffle__.stub_method;
  import __ruffle__.stub_getter;

  [Ruffle(InstanceAllocator)]
  [API("661")]
  public class File extends FileReference
  {
    private static var _applicationStorageDirectory:File;
    private static var _applicationDirectory:File;
    private static var _userDirectory:File;
    private static var _desktopDirectory:File;
    private static var _documentsDirectory:File;

    private static const urlReg:RegExp = /^(?:file|app|app-storage):\/{1,3}(.+)?$/i;

    public function File(path:String = null)
    {
      super();
      if (path)
      {
        var match:Object = urlReg.exec(path);
        if (match)
          this.url = path;
        else
          this.nativePath = path;
      }
    }

    [API("681")]
    public static function get permissionStatus():String
    {
      stub_method("flash.net.FileReference", "permissionStatus");
      return "granted";
    }

    public native function get nativePath():String;
    public native function set nativePath(value:String):void;
    public native function get url():String;
    public native function set url(value:String):void;

    override public native function get name():String;
    override public native function get extension():String;
    public native function get parent():File;

    override public function get type():String
    {
      stub_getter("flash.filesystem.File", "type");
      return extension;
    }

    public native function resolvePath(path:String):File;

    private static native function getUserDirectoryPath():String;
    private static native function getDesktopDirectoryPath():String;
    private static native function getDocumentsDirectoryPath():String;

    public static function get applicationStorageDirectory():File
    {
      if (!_applicationStorageDirectory)
        _applicationStorageDirectory = new File('app-storage:/');
      return _applicationStorageDirectory;
    }

    public static function get applicationDirectory():File
    {
      if (!_applicationDirectory)
        _applicationDirectory = new File('app:/');
      return _applicationDirectory;
    }

    public static function get userDirectory():File
    {
      if (!_userDirectory)
        _userDirectory = new File(getUserDirectoryPath());
      return _userDirectory;
    }

    public static function get desktopDirectory():File
    {
      if (!_desktopDirectory)
        _desktopDirectory = new File(getDesktopDirectoryPath());
      return _desktopDirectory;
    }

    public static function get documentsDirectory():File
    {
      if (!_documentsDirectory)
        _documentsDirectory = new File(getDocumentsDirectoryPath());
      return _documentsDirectory;
    }

    public static function get systemCharset():String
    {
      stub_getter("flash.filesystem.File", "systemCharset");
      return "utf8";
    }

    public static native function get separator():String;

    public static function get lineEnding():String
    {
      return separator == '\\' ? '\r\n' : '\n';
    }

    public function clone():File
    {
      return new File(this.url);
    }

    override public native function get size():Number;
    public native function get exists():Boolean;
    public native function get isDirectory():Boolean;
    public native function get isHidden():Boolean;
    public native function get spaceAvailable():Number;
    public native function copyTo(newLocation:FileReference, overwrite:Boolean = false):void;
    public native function createDirectory():void;
    public native function deleteDirectory(deleteDirectoryContents:Boolean = false):void;
    public native function deleteFile():void;
    public native function getDirectoryListing():Array;
    public native function moveTo(newLocation:FileReference, overwrite:Boolean = false):void;

    public function openWithDefaultApplication():void
    {
      stub_method("flash.filesystem.File", "openWithDefaultApplication");
    }

    [API("668")]
    public function get downloaded():Boolean
    {
      stub_getter("flash.filesystem.File", "downloaded");
    }

    [API("668")]
    public function set downloaded(value:Boolean):void
    {
      stub_setter("flash.filesystem.File", "downloaded");
    }

    // Displays a directory chooser dialog box, in which the user can select a directory.
    public function browseForDirectory(title:String):void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "browseForDirectory");
    }

    // Displays the Open File dialog box, in which the user can select a file to open.
    public function browseForOpen(title:String, typeFilter:Array = null):void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "browseForOpen");
    }

    // Displays the Open File dialog box, in which the user can select one or more files to open.
    public function browseForOpenMultiple(title:String, typeFilter:Array = null):void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "browseForOpenMultiple");
    }

    // Displays the Save File dialog box, in which the user can select a file destination.
    public function browseForSave(title:String):void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "browseForSave");
    }

    // [override] Cancels any pending asynchronous operation.
    override public function cancel():void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "cancel");
    }

    // Canonicalizes the File path.
    public function canonicalize():void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "canonicalize");
    }

    // Begins copying the file or directory at the location specified by this File object to the location specified by the destination parameter.
    public function copyToAsync(newLocation:FileReference, overwrite:Boolean = false):void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "copyToAsync");
    }

    // Returns a reference to a new temporary directory.
    public static function createTempDirectory():File
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "createTempDirectory");
    }

    // Returns a reference to a new temporary file.
    public static function createTempFile():File
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "createTempFile");
    }

    // Deletes the directory asynchronously.
    public function deleteDirectoryAsync(deleteDirectoryContents:Boolean = false):void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "deleteDirectoryAsync");
    }

    // Deletes the file asynchronously.
    public function deleteFileAsync():void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "deleteFileAsync");
    }

    // Asynchronously retrieves an array of File objects corresponding to the contents of the directory represented by this File object.
    public function getDirectoryListingAsync():void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "getDirectoryListingAsync");
    }

    // Returns an array of File objects, listing the file system root directories.
    public static function getRootDirectories():Array
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "getRootDirectories");
    }

    // Begins moving the file or directory at the location specified by this File object to the location specified by the newLocation parameter.
    public function moveToAsync(newLocation:FileReference, overwrite:Boolean = false):void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "moveToAsync");
    }

    // Moves a file or directory to the trash.
    public function moveToTrash():void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "moveToTrash");
    }

    // Asynchronously moves a file or directory to the trash.
    public function moveToTrashAsync():void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "moveToTrashAsync");
    }

    // [override] Requests permission to access filesystem.
    [API("681")]
    override public function requestPermission():void
    {
      // Unknown Implementation
      stub_method("flash.filesystem.File", "requestPermission");
    }

    public function get icon():Icon
    {
      stub_getter("flash.filesystem.File", "icon");
    }

    public function get isPackage():Boolean
    {
      stub_getter("flash.filesystem.File", "isPackage");
    }

    public function get isSymbolicLink():Boolean
    {
      stub_getter("flash.filesystem.File", "isSymbolicLink");
    }
  }
}
