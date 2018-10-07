import 'dart:convert';

class Task {
  int userId;
  int id;
  String text;
  String note;
  String category;
  bool completed;
  String scheduleIntervalType;
  int scheduleIntervalValue;
  DateTime scheduleTime;

  Task({
    this.userId,
    this.id,
    this.text,
    this.note,
    this.category,
    this.completed,
    this.scheduleIntervalType,
    this.scheduleIntervalValue,
    this.scheduleTime,
  });

  factory Task.fromJson(Map<String, dynamic> parsedJson) {
    return Task(
      userId: parsedJson['user_id'],
      id: parsedJson['id'],
      text: parsedJson ['text'],
      note: parsedJson['note'],
      category: parsedJson['category'],
      completed: parsedJson['completed'],
      scheduleIntervalType: parsedJson['schedule_interval_type'],
      scheduleIntervalValue: parsedJson['schedule_interval_value'],
      scheduleTime: DateTime.parse(parsedJson['schedule_time']),
    );
  }

  String toJson() {
    return json.encode({
      "user_id": userId,
      "id": id,
      "text": text,
      "note": note,
      "category": category,
      "completed": completed,
      "schedule_interval_type": scheduleIntervalType,
      "schedule_interval_value": scheduleIntervalValue,
      "schedule_time": scheduleTime.toIso8601String(),
    });
  }

  completeTask() {
    if (completed == true) {
      completed = false;
      return;
    }
    if (scheduleIntervalValue == 0) {
      completed = true;
      return;
    }
    switch(scheduleIntervalType) {
      case 'Day':
        scheduleTime = scheduleTime.add(Duration(days: scheduleIntervalValue));
        break;
      case 'Week':
        scheduleTime = scheduleTime.add(Duration(days: 7 * scheduleIntervalValue));
        break;
      case 'Month':
        scheduleTime = scheduleTime.add(Duration(days: 31 * scheduleIntervalValue));
        break;
      case 'Year':
        scheduleTime = scheduleTime.add(Duration(days: 365 * scheduleIntervalValue));
        break;
    }
    return;
  }
}

