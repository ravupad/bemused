import 'dart:math';

import 'package:flutter/material.dart';

class Test extends StatelessWidget {
  Test({key}): super(key: key);

  @override
  Widget build(BuildContext context) {
    return Center(
        child: CustomPaint(
          size: Size(300, 300),
          painter: ToDoIcon(),
        )
    );
  }
}

class ToDoIcon extends CustomPainter {
  @override
  void paint(Canvas canvas, Size size) {
    var paint = Paint()
      ..strokeWidth = 10
      ..color = Colors.white
      ..strokeCap = StrokeCap.round;
    var p1 = Offset(0, size.height*0.65);
    var p2 = Offset(size.width*0.1, size.height*0.65);
    var p3 = move(p1, p2, 75, 45);
    canvas.drawLine(p2, p3, paint);
    var p4 = move(p2, p3, 200, -90);
    canvas.drawLine(p3, p4, paint);
  }
  @override
  bool shouldRepaint(CustomPainter oldDelegate) {
    return false;
  }
}

Offset move(Offset a, Offset b, double radius, double theta) {
  var radians = (pi*theta)/180;
  var c = Offset(
      b.dx - a.dx,
      b.dy - a.dy);
  var d = Offset(
    c.dx*cos(radians) - c.dy*sin(radians),
    c.dx*sin(radians) + c.dy*cos(radians)
  );
  var r = sqrt(d.dx*d.dx + d.dy*d.dy);
  return Offset(
    b.dx + (radius*d.dx)/r,
    b.dy + (radius*d.dy)/r
  );
}