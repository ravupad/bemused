import 'dart:io';
import 'dart:convert';
import 'package:Bemused/model/task.dart';
import 'package:Bemused/model/error.dart' as E;
import 'package:Bemused/core/storage.dart';

class Http {
  static final String baseUrl = 'https://tasks.ga/api';
  static final String get = "GET";
  static final String post = "POST";
  static final String put = "PUT";
  static final String delete = "DELETE";
  String _authorization;
  final Storage _storage;

  Http(this._storage): _authorization = _storage.getSessionKey();

  Future<HttpClientResponse> _raw(String method, String url, {
    dynamic body,
    bool debug = false,
  }) async {
    if (debug) print('Request: $method | $url | $body');
    var uri = Uri.parse('$baseUrl/$url');
    HttpClientResponse response;
    var request = await HttpClient().openUrl(method, uri);
    request.headers.set("Authorization", _authorization);
    if (body != null) {
      request.write(body);
    }
    response = await request.close();
    return response;
  }

  Future<HttpClientResponse> _status(String method, String url, {
    String body,
    bool debug = false,
  }) async {
    var response = await _raw(method, url,
      body: body, debug: debug);
    if (debug) print('Status Code: ${response.statusCode}');
    if (response.statusCode < 300) {
      return response;
    } else {
      var errorBody = await response.transform(utf8.decoder).join();
      if (debug) print('Error: $errorBody');
      throw E.Error.parseError((json.decode(errorBody) as Map));
    }
  }

  Future<dynamic> _json(String method, String url, {
    String body,
    bool debug = false,
  }) async {
    var response = await _status(method, url, body: body, debug: debug);
    var responseBody = await response.transform(utf8.decoder).join();
    if (debug) print('Response Body: $responseBody');
    return json.decode(responseBody);
  }

  Future<void> checkSession() async {
    await _status(get, 'user/$_authorization');
  }

  Future<void> login(String username, String password) async {
    var response = await _status(put, 'user/$username/$password');
    _authorization = await response.transform(utf8.decoder).join();
    _storage.setSessionKey(_authorization);
  }

  Future<void> signup(String username, String password) async {
    await _status(post, 'user/$username/$password');
  }

  Future<void> logout() async {
    await _status(delete, 'user/$_authorization');
    _authorization = null;
    _storage.setSessionKey(_authorization);
  }

  Future<List<Task>> getTasks() async {
    var list = (await _json(get, "task") as List);
    return list.map((task) {
      return Task.fromJson(task);
    }).toList();
  }

  Future<void> updateTask(Task task) async {
    await _status(put, "task", body: task.toJson());
  }

  Future<void> deleteTask(int id) async {
    await _status(delete, 'task/$id');
  }
}
