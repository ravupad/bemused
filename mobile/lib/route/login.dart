import 'package:Bemused/main.dart';
import 'package:Bemused/route/check_internet.dart';
import 'package:Bemused/route/signup.dart';
import 'package:Bemused/route/tasks.dart';
import 'package:flutter/material.dart';
import 'package:Bemused/model/error.dart' as E;

class Login extends StatefulWidget {
  Login({key}): super(key: key);

  @override
  _LoginState createState() => _LoginState();
}

class _LoginState extends State<Login> {
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
    Widget top = Text("Login",
      style: TextStyle(
        color: Colors.white,
        fontSize: 25,
        fontWeight: FontWeight.w600,
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

  void onSubmit() {
    if (_username.length == 0 || _password.length == 0) {
      setState(() {
        _error = "Enter username and password";
      });
      return;
    }
    _error = null;
    var hidden = {E.ErrorCode.Unknown};
    http.login(_username, _password).then((_) {
      return Navigator.pushReplacement(
          context,
          MaterialPageRoute(builder: (context) => Tasks()));
    }).catchError((ex) {
      if(ex is E.Error) {
        if (ex is E.Error && hidden.contains(ex.errorCode)) {
          _error = 'Internal Server Error';
          setState(() {});
        } else if (ex is E.Error) {
          _error = ex.message;
          setState(() {});
        }
      } else {
        goToCheckInternet(context);
      }
    });
  }

  Widget getSubmitButton(double width, double height) {
    bool isInvalid = _username.length == 0 || _password.length == 0;
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
      width: width,
      height: height,
      color: isInvalid ? Colors.grey : Color(0xFF2F80ED),
    );
    submitButton = GestureDetector(
      child: submitButton,
      onTap: onSubmit,
    );
    return submitButton;
  }

  Widget getBody(double width, double height) {
    double buttonHeight = height*6/100;
    Widget submitButton = getSubmitButton(double.infinity, buttonHeight);
    Widget signupButton = Text(
      "Signup",
      style: TextStyle(
        color: Colors.white,
        fontWeight: FontWeight.w600,
        fontSize: 18,
      ),
    );
    signupButton = Center(child: signupButton);
    signupButton = Container(
      child: signupButton,
      width: double.infinity,
      height: buttonHeight,
      color: Color(0xFF2F80ED),
    );
    signupButton = GestureDetector(
      child: signupButton,
      onTap: () {
        Navigator.pushReplacement(
          context,
          MaterialPageRoute(builder: (context) => Signup()),
        );
      },
    );
    Widget body = Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        TextField(
          onChanged: (text) {
            _username = text;
            setState(() {});
          },
          style: TextStyle(
            fontSize: 18,
            fontWeight: FontWeight.w500,
          ),
          decoration: InputDecoration(
            helperText: "Username"
          ),
        ),
        Container(height: 10),
        TextField(
          obscureText: true,
          onChanged: (text) {
            _password = text;
            setState(() {});
          },
          style: TextStyle(
            fontSize: 18,
            fontWeight: FontWeight.w500,
          ),
          decoration: InputDecoration(
            helperText: 'Password',
          ),
        ),
        Container(height: 20),
        error(),
        Container(height: 30),
        submitButton,
        Container(height: 40),
        signupButton
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
    return (_error == null || _error.length == 0)
        ?
    Container(width: 0, height: 0)
        :
    Text(_error,
      style: TextStyle(
        color: Colors.red,
      ),
    );
  }
}