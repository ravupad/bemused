import 'dart:convert';
import 'dart:io';
import 'package:Bemused/main.dart';
import 'package:Bemused/route/tasks.dart';
import 'package:flutter/material.dart';
import 'package:Bemused/model/error.dart' as E;

import 'login.dart';

class Signup extends StatefulWidget {
  Signup({key}): super(key: key);

  @override
  _SignupState createState() => _SignupState();
}

class _SignupState extends State<Signup> {
  String _username = '';
  String _password = '';
  String _error;

  @override
  Widget build(BuildContext context) {
    double width = MediaQuery.of(context).size.width;
    double height = MediaQuery.of(context).size.height;
    Widget separator = Container(
      height: height*10/100,
    );
    Widget top = Text("Signup",
      style: TextStyle(
        color: Colors.white,
        fontSize: 25,
        fontWeight: FontWeight.w500,
        decoration: TextDecoration.none,
        fontFamily: "Roboto",
      ),
    );
    top = Center(child: top);
    top = Container(
      child: top,
      height: height/10,
      decoration: BoxDecoration(
        color: Color(0xFF2F80ED),
      ),
    );
    List<Widget> components = List();
    components.add(top);
    components.add(separator);
    components.add(getBody(width, height));
    Widget body = ListView(
      children: components,
      scrollDirection: Axis.vertical,
    );
    body = Scaffold(body: body);
    body = SafeArea(child: body);
    return body;
  }

  onSubmit() async {
    try {
      await http.signup(_username, _password);
      Navigator.pushReplacement(
        context,
        MaterialPageRoute(builder: (context) => Login()),
      );
    } on E.Error catch(ex) {
      _error = ex.message;
      setState(() {});
    }
  }

  Widget getBody(double width, double height) {
    double buttonHeight = height*5/100;
    Widget submitButton = Text(
      "Submit",
      style: TextStyle(
        color: Colors.white,
        fontWeight: FontWeight.w600,
        fontSize: 18,
      ),
    );
    submitButton = Center(child: submitButton);
    submitButton = Container(
      child: submitButton,
      width: double.infinity,
      height: buttonHeight,
      color: Color(0xFF2F80ED),
    );
    submitButton = GestureDetector(
      child: submitButton,
      onTap: onSubmit,
    );
    Widget loginButton = Text(
      "Login",
      style: TextStyle(
        color: Colors.white,
        fontWeight: FontWeight.w600,
        fontSize: 18,
      ),
    );
    loginButton = Center(child: loginButton);
    loginButton = Container(
      child: loginButton,
      width: double.infinity,
      height: buttonHeight,
      color: Color(0xFF2F80ED),
    );
    loginButton = GestureDetector(
      child: loginButton,
      onTap: () {
        Navigator.pushReplacement(
          context,
          MaterialPageRoute(builder: (context) => Login()),
        );
      },
    );
    Widget body = Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        Text("Username",
            style: TextStyle(
              fontWeight: FontWeight.w700,
              color: Color(0xFF616161),
              fontSize: 16,
            )
        ),
        Container(height: 10),
        TextField(
          onChanged: (text) {
            _username = text;
          },
          decoration: InputDecoration(),
        ),
        Container(height: 50),
        Text("Password",
            style: TextStyle(
              fontWeight: FontWeight.w700,
              color: Color(0xFF616161),
              fontSize: 16,
            )
        ),
        Container(height: 10),
        TextField(
          obscureText: true,
          onChanged: (text) {
            _password = text;
          },
          decoration: InputDecoration(),
        ),
        Container(height: 50),
        error(),
        Container(height: 50),
        submitButton,
        Container(height: 50),
        loginButton
      ],
    );
    body = Center(child: body);
    double paddingLR = width*40/100;
    body = Container(
      child: body,
      padding: EdgeInsets.fromLTRB(paddingLR/2, 0, paddingLR/2, 0),
    );
    return body;
  }

  Widget error() {
    if (_error == null || _error.compareTo('') == 0) {
      return Container(width: 0, height: 0);
    } else {
      return Text(_error,
        style: TextStyle(
          color: Colors.red,
        ),
      );
    }
  }
}