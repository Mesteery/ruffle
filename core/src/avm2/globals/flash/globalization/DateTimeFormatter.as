package flash.globalization
{
  public class DateTimeFormatter
  {
    public var _requestedLocaleIDName:String;
    public var _pattern:String;

    public function DateTimeFormatter(requestedLocaleIDName:String, dateStyle:String = DateTimeStyle.SHORT, timeStyle:String = DateTimeStyle.NONE)
    {
      _requestedLocaleIDName = requestedLocaleIDName;
    }

    public function get requestedLocaleIDName():String
    {
      return _requestedLocaleIDName;
    }

    public function get actualLocaleIDName():String
    {
      return _requestedLocaleIDName;
    }

    public function setDateTimePattern(pattern:String):void
    {
      _pattern = pattern;
    }

    public function formatUTC(dateTime:Date):String
    {
      return formatUTCInternal(dateTime, _requestedLocaleIDName, _pattern);
    }

    public native function formatUTCInternal(dateTime:Date, locale:String):String;

    public function getMonthNames(nameStyle:String = "full", context:String = "standalone"):Vector.<String>
    {
      return new <String>["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    }
  }
}