import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:Bemused/model/task.dart' as M;
import 'package:intl/intl.dart';

class Task extends StatefulWidget {
  final M.Task task;

  Task(this.task, {key}): super(key: key);

  @override
  _Task createState() => _Task(task);
}

class _Task extends State<Task> with RouteAware {
  M.Task task;

  _Task(this.task);

  @override
  Widget build(BuildContext context) {
    Widget header = Text('Task',
      style: TextStyle(
        color: Colors.white,
        fontSize: 25,
        decoration: TextDecoration.none
      ),
      textAlign: TextAlign.center,
    );
    header = Container(
      child: header,
      padding: EdgeInsets.fromLTRB(0, 20, 0, 20),
      width: double.infinity,
      decoration: BoxDecoration(
        color: Color(0XFF2F80ED),
      ),
    );
    Widget text = TextField(
      controller: TextEditingController()..text = task.text,
      decoration: InputDecoration(
          helperText: "Text"
      ),
    );
    Widget note = TextField(
      controller: TextEditingController()..text = task.note,
      maxLines: 3,
      decoration: InputDecoration(
          helperText: "Note"
      ),
    );
    DateFormat format = DateFormat('hh:mm a dd MMM yyyy');
    Widget time = TextField(
      controller: TextEditingController()..text = format.format(task.scheduleTime.toLocal()),
      decoration: InputDecoration(
          helperText: "Time"
      ),
    );
    Widget repeatValue = TextField(
      controller: TextEditingController()..text = task.scheduleIntervalValue.toString(),
      keyboardType: TextInputType.number,
      decoration: InputDecoration(
          helperText: "Repeat"
      ),
    );
    repeatValue = Container(
      child: repeatValue,
      width: 50,
      height: 50,
    );
    Widget repeatPeriod = TextField(
      controller: TextEditingController()..text = task.scheduleIntervalType,
      decoration: InputDecoration(
          helperText: "After"
      ),
    );
    repeatPeriod = Container(
      child: repeatPeriod,
      width: 100,
      height: 50,
    );
    Widget repeat = Row(
      children: <Widget>[
        repeatValue,
        Container(width: 50),
        repeatPeriod,
      ],
    );
    Widget body = Column(
      children: <Widget>[
        Container(height: 50),
        text,
        Container(height: 30),
        note,
        Container(height: 40),
        time,
        Container(height: 40),
        repeat,
      ],
    );
    body = Container(
      child: body,
      width: 300,
      padding: EdgeInsets.fromLTRB(50, 0, 50, 0),
    );
    body = ListView(
      children: <Widget>[
        header,
        body,
      ],
    );
    body = Container(
      child: body,
      decoration: BoxDecoration(
        color: Colors.white,
      ),
    );
    body = Scaffold(
      body: body,
    );
    body = SafeArea(
      child: body
    );
    return body;
  }
}