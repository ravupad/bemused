class Error {
  ErrorCode errorCode;
  String message;

  Error(this.errorCode, this.message);

  factory Error.parseError(Map<String, dynamic> json) {
    String message = json['message'];
    ErrorCode errorCode = ErrorCode.Unknown;
    switch(json['error_code']) {
      case 'NotAuthenticated': errorCode = ErrorCode.NotAuthenticated; break;
      case 'UserNotFound': errorCode = ErrorCode.UserNotFound; break;
      case 'WrongPassword': errorCode = ErrorCode.WrongPassword; break;
      case 'UserNameTaken': errorCode = ErrorCode.UserNameTaken; break;
      default: message = json.toString();
    }
    return Error(errorCode, message);
  }
}

enum ErrorCode {
  Unknown,
  NotAuthenticated,
  UserNotFound,
  WrongPassword,
  UserNameTaken,
}