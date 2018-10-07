import 'dart:io';
import 'dart:math';
import 'package:Bemused/main.dart';
import 'package:Bemused/model/error.dart' as M;
import 'package:Bemused/model/task.dart' as M;
import 'package:Bemused/route/check_internet.dart';
import 'package:Bemused/route/task.dart';
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:flutter_svg/flutter_svg.dart';

class Tasks extends StatefulWidget {
  Tasks({key}): super(key: key);

  @override
  _Tasks createState() => _Tasks();
}

class _Tasks extends State<Tasks> with RouteAware {
  List<M.Task> _tasks = List();
  bool showCategory = false;
  Set<String> allCategories = Set();
  Set<String> chosenCategories = Set();
  bool showAll = true;
  bool showCompleted = false;
  List<M.Task> past = List();
  List<M.Task> yesterday = List();
  List<M.Task> today = List();
  List<M.Task> tomorrow = List();
  List<M.Task> future = List();
  bool fetchTaskInProgress = true;
  bool taskUpdateInProgress = false;

  @override
  void initState() {
    withFetchTaskInProgress(init);
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

  Future<void> init() async {
    try {
      var tasks = await http.getTasks();
      _tasks = tasks;
      allCategories.clear();
      for (var i = 0; i < _tasks.length; i++) {
        allCategories.add(_tasks[i].category);
      }
      calculateTasks();
    } on M.Error {

    } on SocketException {
      await goToCheckInternet(context);
    }
  }

  void calculateTasks() {
    var tasks = _tasks.where((task) {
      return task.completed == showCompleted;
    }).where((task) {
      return showAll || chosenCategories.contains(task.category);
    }).toList();
    tasks.sort((t1, t2) {
      return t1.scheduleTime
          .difference(t2.scheduleTime)
          .inMilliseconds;
    });
    DateTime now = DateTime.now();
    DateTime todayStart = DateTime(now.year, now.month, now.day);
    DateTime yesterdayStart = todayStart.add(Duration(days: -1));
    DateTime tomorrowStart = todayStart.add(Duration(days: 1));
    DateTime futureStart = tomorrowStart.add(Duration(days: 1));
    past = tasks.where((t) {
      return t.scheduleTime.isBefore(yesterdayStart);
    }).toList();
    yesterday = tasks.where((t) {
      return t.scheduleTime.isAfter(yesterdayStart) &&
        t.scheduleTime.isBefore(todayStart);
    }).toList();
    today = tasks.where((t) {
      return t.scheduleTime.isAfter(todayStart) &&
        t.scheduleTime.isBefore(tomorrowStart);
    }).toList();
    tomorrow = tasks.where((t) {
      return t.scheduleTime.isAfter(tomorrowStart) &&
          t.scheduleTime.isBefore(futureStart);
    }).toList();
    future = tasks.where((t) {
      return t.scheduleTime.isAfter(futureStart);
    }).toList();
  }

  Future<dynamic> withFetchTaskInProgress(Future<dynamic> f()) async {
    setState(() {
      fetchTaskInProgress = true;
    });
    var result = await f();
    setState(() {
      fetchTaskInProgress = false;
    });
    return result;
  }

  Future<void> withTaskUpdateInProgress(Future<void> f()) async {
    setState(() {
      taskUpdateInProgress = true;
    });
    await f();
    setState(() {
      taskUpdateInProgress = false;
    });
  }

  Widget getHeader() {
    Widget tasksHeaderText = Text('Tasks',
      style: TextStyle(
        fontSize: 25,
        color: Colors.white,
        decoration: TextDecoration.none,
      ),
      textAlign: TextAlign.center,
    );
    tasksHeaderText = Container(
      child: tasksHeaderText,
      alignment: Alignment.center,
      width: double.infinity,
      height: 40,
    );
    Widget refresh = SvgPicture.asset('assets/icons/refresh.svg',
        width: 36,
        height: 36
    );
    refresh = GestureDetector(
      child: refresh,
      behavior: HitTestBehavior.opaque,
      onTap: () {
        withFetchTaskInProgress(init);
      },
    );
    refresh = Container(
      child: refresh,
      width: double.infinity,
      alignment: Alignment.centerRight,
    );
    List<Widget> headerChildren = [
      tasksHeaderText,
      refresh,
    ];
    Widget header = Stack(
      children: headerChildren,
    );
    header = Container(
      child: header,
      padding: EdgeInsets.fromLTRB(10, 10, 10, 10),
      decoration: BoxDecoration(
        color: Color(0xFF2F80ED),
      ),
    );
    return header;
  }

  Widget getFilter() {
    Widget all = Text('All',
      overflow: TextOverflow.ellipsis,
      textAlign: TextAlign.center,
      style: TextStyle(
        color: showAll ? Color(0xFF009688) : Colors.black,
        fontWeight: FontWeight.w700,
        fontSize: 20,
        fontFamily: "Roboto",
        decoration: TextDecoration.none,
      ),
    );
    all = Container(
      child: all,
      padding: EdgeInsets.fromLTRB(0, 10, 0, 10),
    );
    all = GestureDetector(
      child: all,
      behavior: HitTestBehavior.opaque,
      onTap: () {
        setState(() {
          showAll = !showAll;
          calculateTasks();
        });
      },
    );
    all = Expanded(
      child: all,
      flex: 1,
    );
    Widget completedFilter = Text("Completed",
      textAlign: TextAlign.center,
      overflow: TextOverflow.ellipsis,
      style: TextStyle(
        color: this.showCompleted ? Color(0xFF009688) : Colors.black,
        fontWeight: FontWeight.w700,
        fontSize: 20,
        fontFamily: "Roboto",
        decoration: TextDecoration.none,
      ),
    );
    completedFilter = Container(
      child: completedFilter,
      padding: EdgeInsets.fromLTRB(0, 10, 0, 10),
    );
    completedFilter = GestureDetector(
      child: completedFilter,
      behavior: HitTestBehavior.opaque,
      onTap: () {
        setState(() {
          this.showCompleted = !this.showCompleted;
          calculateTasks();
        });
      },
    );
    completedFilter = Expanded(
      child: completedFilter,
      flex: 1,
    );
    Widget archivedFilter = Text("Archived",
      textAlign: TextAlign.center,
      overflow: TextOverflow.ellipsis,
      style: TextStyle(
        color: Colors.black,
        fontWeight: FontWeight.w700,
        fontSize: 20,
        fontFamily: "Roboto",
        decoration: TextDecoration.none,
      ),
    );
    archivedFilter = Expanded(
      child: archivedFilter,
      flex: 1,
    );
    Widget filter = Container(
      child: Row(
        children: <Widget>[
          all,
          completedFilter,
          archivedFilter,
        ],
      ),
      decoration: BoxDecoration(
        color: Colors.white,
        border: Border(
          bottom: BorderSide(
            color: Colors.black,
            width: 1,
          ),
        ),
      ),
      padding: EdgeInsets.fromLTRB(10, 5, 10, 5),
    );
    return filter;
  }

  Widget getCategoryButton(String category) {
    bool chosen = chosenCategories.contains(category);
    Widget categoryButton = Text(category,
      style: TextStyle(
        color: chosen ? Color(0xFF5322A4) : Colors.black,
        fontWeight: chosen ? FontWeight.w800 : FontWeight.w500,
        fontSize: 15,
        fontFamily: "Roboto",
        decoration: TextDecoration.none,
      ),
    );
    categoryButton = Center(
      child: categoryButton,
    );
    categoryButton = Container(
      child: categoryButton,
      width: 100,
      margin: EdgeInsets.fromLTRB(15, 10, 15, 10),
      padding: EdgeInsets.only(top: 4, bottom: 4),
      decoration: BoxDecoration(
          color: Colors.white,
          borderRadius: BorderRadius.all(Radius.circular(1)),
          boxShadow: <BoxShadow>[
            BoxShadow(
              color: Color(0x99000000),
              offset: Offset(0, 2),
              blurRadius: 4,
            ),
          ]
      ),
    );
    categoryButton = GestureDetector(
      child: categoryButton,
      behavior: HitTestBehavior.opaque,
      onTap: () {
        setState(() {
          if (chosenCategories.contains(category)) {
            chosenCategories.remove(category);
          } else {
            chosenCategories.add(category);
          }
          calculateTasks();
        });
      },
    );
    return categoryButton;
  }

  Widget getCategoriesPicker() {
    List<Widget> categoryTexts = [];
    for (var i = 0; i < allCategories.length; i++) {
      String category = allCategories.elementAt(i);
      Widget categoryButton = getCategoryButton(category);
      categoryTexts.add(categoryButton);
    }
    Widget categoriesPicker = Wrap(
      children: categoryTexts,
      direction: Axis.horizontal,
    );
    categoriesPicker = Container(
      child: categoriesPicker,
      decoration: BoxDecoration(
        color: Colors.white,
        border: Border(
          bottom: BorderSide(
            color: Colors.black,
            width: 1,
          ),
        ),
      ),
    );
    return categoriesPicker;
  }

  Widget getAddButton() {
    Widget button = CustomPaint(
      painter: NewIcon(),
      size: Size(60, 60),
    );
    Widget container = Positioned(
      child: button,
      bottom: 20,
      right: 20,
    );
    return container;
  }

  Widget getTaskRow(M.Task task) {
    double width = MediaQuery.of(context).size.width;
    double todoPaddingWidth = 5.0*2;
    double todoIconWidth = 16;
    double taskDescriptionWidth = 80;
    double taskDecoratorWidth = 10;
    double totalWidth =
        todoPaddingWidth +
        todoIconWidth +
        taskDescriptionWidth +
        taskDecoratorWidth;
    Widget completeIcon = CustomPaint(
      size: Size(90, 90),
      painter: ToDoIcon(task.completed),
    );
    completeIcon = Container(
      child: completeIcon,
      width: width*todoIconWidth/totalWidth,
      height: 40,
    );
    completeIcon = GestureDetector(
      child: completeIcon,
      onTap: () => withTaskUpdateInProgress(() async {
        task.completeTask();
        await http.updateTask(task);
        calculateTasks();
      }),
    );
    Widget taskText = Text(
      task.text,
      overflow: TextOverflow.ellipsis,
      style: TextStyle(
        fontSize: 16,
        fontWeight: FontWeight.w700,
        fontFamily: 'Roboto',
        decoration: TextDecoration.none,
        color: Colors.black,
      ),
    );
    Widget taskCategory = Expanded(
      flex: 25,
      child: Text(task.category,
        overflow: TextOverflow.ellipsis,
        style: TextStyle(
          fontSize: 14,
          fontWeight: FontWeight.w600,
          fontFamily: 'Roboto',
          decoration: TextDecoration.none,
          color: Colors.black,
        ),
      ),
    );
    DateFormat format = DateFormat('hh:mm a dd MMM yyyy');
    Widget taskTime = Expanded(
      flex: 60,
      child: Text(format.format(task.scheduleTime.toLocal()),
        overflow: TextOverflow.ellipsis,
        style: TextStyle(
          fontSize: 14,
          fontWeight: FontWeight.w600,
          fontFamily: 'Roboto',
          decoration: TextDecoration.none,
          color: Colors.black,
        ),
      ),
    );
    Widget taskDescriptionFooter = Row(
      children: <Widget>[
        taskCategory,
        taskTime,
      ],
    );
    taskDescriptionFooter = Container(
      child: taskDescriptionFooter,
      padding: EdgeInsets.only(top: 5),
    );
    List<Widget> taskDescriptionItems = <Widget>[];
    taskDescriptionItems.add(taskText);
    taskDescriptionItems.add(Container(height: 10));
    taskDescriptionItems.add(taskDescriptionFooter);
    Widget taskDescription = Column(
      children: taskDescriptionItems,
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
    );
    taskDescription = Container(
      child: taskDescription,
      padding: EdgeInsets.only(left: 15),
      width: width * taskDescriptionWidth / totalWidth,
    );
    Widget note = Text("#",
      style: TextStyle(
        fontSize: 12,
        color: Colors.green,
      ),
      textAlign: TextAlign.center,
    );
    List<Widget> children = List();
    if (task.note != null &&  task.note != '') {
      children.add(note);
    }
    Widget decorators = Column(
      children: children,
    );
    decorators = Container(
      child: decorators,
      width: width * taskDecoratorWidth / totalWidth,
    );
    Widget taskRow = Row(
      children: <Widget>[
        completeIcon,
        taskDescription,
        decorators
      ],
    );
    taskRow = Container(
      child: taskRow,
      padding: EdgeInsets.fromLTRB(
          width * todoPaddingWidth / (2 * totalWidth),
          15,
          width * todoPaddingWidth / (2 * totalWidth),
          15),
      margin: EdgeInsets.fromLTRB(0, 0, 0, 15),
      decoration: BoxDecoration(
        color: Colors.white,
        boxShadow: <BoxShadow>[
          BoxShadow(
            color: Color(0x88000000),
            blurRadius: 2
          ),
        ],
      ),
    );
    taskRow = GestureDetector(
      child: taskRow,
      onTap: () {
        Navigator.push(
          context,
          MaterialPageRoute(
            builder: (context) => Task(task),
          )
        );
      },
    );
    return taskRow;
  }

  Widget getTaskSection(String title, Color titleColor, List<M.Task> tasks) {
    Widget header = Text(title,
      style: TextStyle(
        color: titleColor,
        fontSize: 20,
        decoration: TextDecoration.none,
      ),
      textAlign: TextAlign.left,
    );
    header = Container(
      child: header,
      padding: EdgeInsets.fromLTRB(0, 15, 0, 25),
    );
    List<Widget> taskRows = List();
    taskRows.add(header);
    for(int i = 0; i < tasks.length; i++) {
      M.Task task = tasks[i];
      Widget taskRow = getTaskRow(task);
      taskRows.add(taskRow);
    }
    Widget taskList = Column(
      children: taskRows,
    );
    return taskList;
  }

  List<Widget> getTaskSections() {
    List<Widget> sections = List();
    if (past.length > 0) {
      sections.add(getTaskSection('Past', Color(0xFFD20000), past));
    }
    if (yesterday.length > 0) {
      sections.add(getTaskSection('Yesterday', Color(0xFFF2994A), yesterday));
    }
    if (today.length > 0) {
      sections.add(getTaskSection('Today', Color(0xFF2F80ED), today));
    }
    if (tomorrow.length > 0) {
      sections.add(getTaskSection('Tomorrow', Color(0xFF27AE60), tomorrow));
    }
    if (future.length > 0) {
      sections.add(getTaskSection('Future', Color(0xFFBB6BD9), future));
    }
    return sections;
  }

  @override
  Widget build(BuildContext context) {
    double height = MediaQuery.of(context).size.height;
    Widget header = getHeader();
    Widget filter = getFilter();
    Widget categoriesPicker = getCategoriesPicker();
    Widget taskLoading = Text("Tasks are being fetched",
      textAlign: TextAlign.center,
      style: TextStyle(
        fontSize: 25,
        fontWeight: FontWeight.bold,
        fontFamily: 'Roboto',
        decoration: TextDecoration.none,
        color: Colors.grey,
      ),
    );
    taskLoading = Container(
      child: taskLoading,
      padding: EdgeInsets.only(top: max(10, height/2 - 150)),
    );
    Widget taskUpdating = Text("Task is being updated",
      textAlign: TextAlign.center,
      style: TextStyle(
        fontSize: 25,
        fontWeight: FontWeight.bold,
        fontFamily: 'Roboto',
        decoration: TextDecoration.none,
        color: Colors.grey,
      ),
    );
    taskUpdating = Container(
      child: taskUpdating,
      padding: EdgeInsets.only(top: max(10, height/2 - 150)),
    );
    List<Widget> bodyChildren = [];
    bodyChildren.add(header);
    bodyChildren.add(filter);
    if (!showAll) {
      bodyChildren.add(categoriesPicker);
    }
    if (fetchTaskInProgress) {
      bodyChildren.add(taskLoading);
    } else if (taskUpdateInProgress) {
      bodyChildren.add(taskUpdating);
    }
    else {
      bodyChildren.addAll(getTaskSections());
    }
    Widget body = ListView(
      children: bodyChildren,
    );
    body = Container(
      child: body,
      color: Colors.white,
    );
    Widget addButton = getAddButton();
    body = Stack(
      children: <Widget>[
        body,
        addButton
      ],
    );
    body = SafeArea(
      child: body,
    );
    return body;
  }
}

class NewIcon extends CustomPainter {
  NewIcon();

  @override
  void paint(Canvas canvas, Size size) {
    Offset center = Offset(size.width/2, size.height/2);
    double radius = min(size.width/2, size.height/2);
    var paint = Paint()
      ..color = Color(0xFF27AE60);
    var shadowPaint = Paint()
      ..color = Color(0xDD111111)
      ..maskFilter = MaskFilter.blur(BlurStyle.normal, 6);
    canvas.drawCircle(
        center,
        radius,
        shadowPaint);
    canvas.drawCircle(
        center,
        radius,
        paint);
    paint.color = Colors.white;
    paint.strokeWidth = radius/6;
    canvas.drawLine(
        Offset(center.dx, size.height*1/4),
        Offset(center.dx, size.height*3/4),
        paint);
    canvas.drawLine(
        Offset(size.width*1/4, center.dy),
        Offset(size.width*3/4, center.dy),
        paint);
  }

  @override
  bool shouldRepaint(CustomPainter oldDelegate) {
    return false;
  }
}

class ToDoIcon extends CustomPainter {
  bool drawTick;

  ToDoIcon(this.drawTick);

  @override
  void paint(Canvas canvas, Size size) {
    var h = size.height/40;
    var w = size.width/40;
    var paint = Paint()
      ..color = Colors.white;
    var shadowPaint = Paint()
      ..color = Color(0x77000000)
      ..maskFilter = MaskFilter.blur(BlurStyle.normal, 3);
    canvas.drawCircle(
        Offset(size.width/2, size.height/2 + 1),
        min(size.width/2, size.height/2),
        shadowPaint);
    canvas.drawCircle(
        Offset(size.width/2, size.height/2),
        min(size.width/2, size.height/2),
        paint);
    if (drawTick) {
      paint.color = Color(0xFF3B22D6);
      var path = Path();
      path.lineTo(w*8, h*24);
      path.lineTo(w*16, h*32);
      path.lineTo(w*31, h*11);
      path.lineTo(w*28, h*8);
      path.lineTo(w*15, h*26);
      path.lineTo(w*11, h*21);
      path.lineTo(w*8, h*24);
      path.close();
      canvas.drawPath(path, paint);
    }
  }

  @override
  bool shouldRepaint(CustomPainter oldDelegate) {
    return false;
  }
}
