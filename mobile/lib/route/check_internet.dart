import 'package:Bemused/main.dart';
import 'package:flutter/material.dart';

Future<dynamic> goToCheckInternet(BuildContext context) async {
  return await Navigator.push(
    context,
    MaterialPageRoute(builder: (context) => CheckInternet()),
  );
}

class CheckInternet extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.white,
      ),
      child: Center(
        child: GestureDetector(
          child: Container(
            padding: EdgeInsets.all(10),
            decoration: BoxDecoration(
              color: Colors.amberAccent,
            ),
            child: Text("Tap To Check Internet Connection",
              textAlign: TextAlign.center,
              style: TextStyle(
                fontSize: 18,
                decoration: TextDecoration.none,
                color: Colors.black,
              ),
            )
          ),
          onTap: () async {
            await checkConnection(context);
          },
        ),
      ),
    );
  }

  checkConnection(BuildContext context) async {
    await http.checkSession().then((_) {
      Navigator.pop(context);
    }).catchError((ex){});
  }
}
