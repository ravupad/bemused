import 'package:shared_preferences/shared_preferences.dart';

enum Keys {
  SessionId,
}

class Storage {
  SharedPreferences _prefs;

  Storage(this._prefs);

  String getSessionKey() {
    return _prefs.getString(Keys.SessionId.toString());
  }

  void setSessionKey(String sessionId) {
    _prefs.setString(Keys.SessionId.toString(), sessionId);
  }
}


