import 'package:Bemused/core/http.dart';
import 'package:Bemused/core/storage.dart';
import 'package:Bemused/main.dart';
import 'package:Bemused/model/error.dart' as E;
import 'package:Bemused/model/error.dart';
import 'package:Bemused/route/tasks.dart';
import 'package:flutter/material.dart';
import 'package:Bemused/route/login.dart';
import 'package:shared_preferences/shared_preferences.dart';

class Splash extends StatefulWidget {
  Splash({key}): super(key: key);
  @override
  _SplashState createState() => _SplashState();
}

class _SplashState extends State<Splash> with RouteAware {
  @override
  void initState() {
    setup().then((_) {
      checkLogin();
    });
    super.initState();
  }

  @override
  void didChangeDependencies() {
    routeObserver.subscribe(this, ModalRoute.of(context));
    super.didChangeDependencies();
  }

  @override
  void dispose() {
    routeObserver.unsubscribe(this);
    super.dispose();
  }

  void didPopNext() {
    checkLogin();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.white,
      ),
      child: Center(
        child: Text("Bemused",
          style: TextStyle(
            decoration: TextDecoration.none,
            color: Colors.blue,
          ),
        ),
      ),
    );
  }

  Future<void> setup() async {
    preferences = await SharedPreferences.getInstance();
    storage = Storage(preferences);
    http = Http(storage);
  }

  Future<void> checkLogin() {
    return http.checkSession().then((_) {
      Navigator.push(
        context,
        MaterialPageRoute(builder: (context) => Tasks()),
      );
    }).catchError(dhhec(E.ErrorCode.NotAuthenticated, (ex) {
      Navigator.push(
        context,
        MaterialPageRoute(builder: (context) => Login()),
      );
    }));
  }

  dhhec(ErrorCode ec, dynamic h(ex)) => (ex) => hhec(context, ex, ec, h);
}