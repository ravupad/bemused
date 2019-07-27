<template>
<div class="tasks flex-column">
  <div class="filters flex-row border">
    <button class="button" v-on:click="showAll = !showAll; computeDisplayedTasks()"
      v-bind:class="{is: showAll}">
      All
    </button>
    <button class="button" v-on:click="toggleShowCompleted()"
      v-bind:class="{is: showCompleted == true}">
      Completed
    </button>
  </div>
  <div class="filters flex-row border" v-if="!showAll">
    <button class="button" v-for="c in categories().keys()" 
      :key="c" v-on:click="updateCategory(c)"
      v-bind:class="{is: category.has(c)}">
      {{c}}
    </button>    
  </div>
  <TaskComponent v-for="t in displayedTasks" :key="t.id"
    :task="t" :selected="isSelected(t)" :setIsEditing="setIsEditing" 
    :event="getEvent(t)" :select="selectTask(t)"
    v-on:update="updateTask" v-on:delete="deleteTask"/>
  <TaskComponent class='new-task'
    :task="task" :selected="isSelected(task)" :setIsEditing="setIsEditing" 
    :event="getEvent(task)" v-on:update="createTask" :select="selectTask(task)"/>
</div>
</template>

<script lang="ts">
import {Component, Vue} from 'vue-property-decorator';
import {createTask, deleteTask, getTasks, updateTask} from '../client';
import TaskComponent from '../components/Task.vue';
import {newTask, Task} from '../models/Task';
import {DateTime} from 'luxon';

@Component({
  components: {
    TaskComponent,
  },
})
export default class TaskView extends Vue {
  private tasks: Task[] = [];
  private task = newTask('Task');
  private category: Set<string> = new Set();
  private showAll = true;
  private showCompleted = false;
  private selectedTask: Task = this.task;
  private displayedTasks: Task[] = [];
  private taskOrderMap: Map<Task, number> = new Map();
  private isEditing: boolean = false;
  private event: string = 'none';
  private eventId: number = 0;

  private mounted() {
    getTasks()
        .then((res) => {
          return res.data;
        })
        .then((tasks) => {
          return tasks.sort((a: Task, b: Task) => {
            return (DateTime.fromISO(a.schedule_time).toMillis()) -
              (DateTime.fromISO(b.schedule_time).toMillis());
          });
        })
        .then((tasks) => this.tasks = tasks)
        .then(() => this.task = newTask('Task'))
        .then(() => this.computeDisplayedTasks())
        .then(() => {
          if (this.displayedTasks.length > 0) {
            this.selectedTask = this.displayedTasks[0];
          }
        });
    document.addEventListener('keydown', this.onKeyDown);
  }

  private beforeDestroy() {
    document.removeEventListener('keydown', this.onKeyDown);
  }

  private onKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      this.event = 'escape';
      this.eventId = this.eventId + 1;
      return;
    }
    if (this.isEditing === true) {
      return;
    }
    if (event.key === 'j') {
      if (this.displayedTasks.length === 0) {
        this.selectedTask = this.task;
        return;
      }
      let idx = this.taskOrderMap.get(this.selectedTask);
      if (idx === undefined) {
        idx = 0;
      } else {
        idx = idx + 1;
      }
      if (idx === this.displayedTasks.length) {
        this.selectedTask = this.task;
      } else {
        this.selectedTask = this.displayedTasks[idx];
      }
    } else if (event.key === 'k') {
      if (this.displayedTasks.length === 0) {
        this.selectedTask = this.task;
        return;
      }
      let idx = this.taskOrderMap.get(this.selectedTask);
      if (idx === undefined) {
        idx = this.displayedTasks.length - 1;
      } else {
        idx = idx - 1;
      }
      if (idx === -1) {
        this.selectedTask = this.task;
      } else {
        this.selectedTask = this.displayedTasks[idx];
      }
    } else if (event.key === 'c') {
      this.event = 'complete';
      this.eventId = this.eventId + 1;
    } else if (event.key === 'e') {
      this.event = 'expand';
      this.eventId = this.eventId + 1;
    } else if (event.key === 't') {
      this.event = 'text';
      this.eventId = this.eventId + 1;
      event.preventDefault();
    } else if (event.key === 'n') {
      this.event = 'note';
      this.eventId = this.eventId + 1;
      event.preventDefault();
    } else if (event.key === 's') {
      this.event = 'stime';
      this.eventId = this.eventId + 1;
      event.preventDefault();
    } else if (event.key === 'u') {
      this.event = 'update';
      this.eventId = this.eventId + 1;
    } else if (event.key === 'd') {
      this.event = 'delete';
      this.eventId = this.eventId + 1;
    } else if (event.key === 'r') {
      this.event = 'repeat';
      this.eventId = this.eventId + 1;
    }
  }

  private getEvent(task: Task): any[] {
    if (this.selectedTask === task) {
      return [this.event, this.eventId];
    } else {
      return ['none', this.eventId];
    }
  }

  private selectTask(task: Task) {
    return () => {
      this.selectedTask = task;
    };
  }

  private setIsEditing(value: boolean) {
    this.isEditing = value;
  }

  private toggleShowCompleted() {
    this.showCompleted = !this.showCompleted;
    this.computeDisplayedTasks();
  }

  private updateCategory(category: string) {
    if (this.category.has(category)) {
      this.category.delete(category);
    } else {
      this.category.add(category);
    }
    this.computeDisplayedTasks();
  }

  private categories(): Set<string> {
    const categories = new Set<string>();
    this.tasks.forEach((task) => categories.add(task.category));
    return categories;
  }

  private computeDisplayedTasks(): Task[] {
    let tasks = this.tasks.filter(() => true);
    if (this.showCompleted) {
      tasks = tasks.filter((t) => t.completed);
    } else {
      tasks = tasks.filter((t) => !t.completed);
    }
    if (!this.showAll) {
      tasks = tasks.filter((task) => this.category.has(task.category));
    }
    tasks = tasks.sort((a: Task, b: Task) => {
      return (DateTime.fromISO(a.schedule_time).toMillis()) -
        (DateTime.fromISO(b.schedule_time).toMillis());
    });
    this.displayedTasks = [];
    this.taskOrderMap = new Map();
    let selectedTaskInDisplay = false;
    for (let i: number = 0; i < tasks.length; i++) {
      this.displayedTasks[i] = tasks[i];
      this.taskOrderMap.set(tasks[i], i);
      if (this.selectedTask.id === tasks[i].id) {
        selectedTaskInDisplay = true;
        this.selectedTask = tasks[i];
      }
    }
    if (selectedTaskInDisplay === false) {
      if (tasks.length === 0) {
        this.selectedTask = this.task;
      } else {
        this.selectedTask = tasks[0];
      }
    }
    return tasks;
  }

  private isSelected(task: Task): boolean {
    return this.selectedTask === task;
  }

  private updateTask(task: Task) {
    updateTask(task)
        .then(() => {
          this.tasks = this.tasks.map((t) => {
            if (t.id === task.id) {
              return task;
            } else {
              return t;
            }
          });
        })
        .then(() => this.computeDisplayedTasks());
  }

  private deleteTask(taskId: number) {
    deleteTask(taskId)
      .then(() => this.tasks = this.tasks.filter((t) => t.id !== taskId))
      .then(() => this.computeDisplayedTasks());
  }

  private createTask(nTask: Task) {
    createTask(nTask)
        .then((res) => {
          nTask.id = res.data;
          this.tasks.push(nTask);
          this.task = newTask('Task');
        })
        .then(() => this.computeDisplayedTasks());
  }
}
</script>

<style scoped>
.tasks {
  width: 900px;
  margin: 0 auto;
}

.filters {
  width: 100%;
  background-color: #b1f2be;
  padding: 10px 0;
  margin-bottom: 15px;
}

.filters > .button {
  margin: 0 10px 0 10px;
  background-color: white;
}

.filters > .is {
  background-color: #4b6db0;
  color: white;
}

select {
  margin-left: 15px;
  display: inline;
  background-color: ghostwhite;
}

.tasks >>> .new-task {
  background-color: #b1f2be;
  margin-top: 20px;
  margin-bottom: 20px;
}
</style>
