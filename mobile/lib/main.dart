import 'dart:io';
import 'package:Bemused/core/http.dart';
import 'package:Bemused/route/check_internet.dart';
import 'package:Bemused/core/storage.dart';
import 'package:flutter/material.dart';
import 'package:Bemused/route/splash.dart';
import 'model/error.dart';
import 'package:shared_preferences/shared_preferences.dart';

SharedPreferences preferences;
Storage storage;
Http http;
RouteObserver<PageRoute> routeObserver = new RouteObserver<PageRoute>();

void main() async {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Bemused',
      navigatorObservers: <NavigatorObserver>[
        routeObserver
      ],
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: Splash(),
    );
  }
}

dynamic hhec(
    BuildContext context,
    Object ex,
    ErrorCode ec,
    dynamic h(Error ex))
{
  return hhe(context, ex, ecs: {ec}, h: h);
}

dynamic hhe(BuildContext context, Object ex, {
  Set<ErrorCode> ecs,
  dynamic h(Error ex)
})
{
  if (ex is SocketException) {
    goToCheckInternet(context);
    throw ex;
  } else if (h != null && ex is Error && (ecs == null ||
      ecs.contains(ex.errorCode))) {
    return h(ex);
  } else {
    throw ex;
  }
}
