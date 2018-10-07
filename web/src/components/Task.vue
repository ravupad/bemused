<template>
<div tabindex="-1" ref="container" class="task border"
  v-bind:class="containerClass()" v-on:click="select()">
  <div class="flex-row">
    <input tabindex="-1" ref="text" class="text" 
      v-model="itask.text" v-on:input="dirty=true"
      v-on:focus="() => setIsEditing(true)" v-on:blur="() => setIsEditing(false)"/>
    <div class="showDetail" v-show="!detail" v-on:click="detail=true"></div>
    <div class="hideDetail" v-show="detail" v-on:click="detail=false"></div>
    <div class="complete" v-bind:class="{is: itask.completed}" v-on:click="complete"></div>
    <input tabindex="-1" ref="stime" class="scheduleTime" v-model="scheduleTime" 
      v-on:input="dirty=true" v-on:keyup.enter="updateTask"
      v-on:focus="() => setIsEditing(true)" v-on:blur="() => setIsEditing(false)"/>
  </div>
  <div v-show="detail">
    <textarea tabindex="-1" ref="note" v-show="showNote" class="note border" v-model="itask.note" 
      v-on:input="dirty=true;textAreaResize()" v-on:mouseover="textAreaResize" 
      v-on:focus="() => setIsEditing(true)" v-on:blur="() => setIsEditing(false)"/>
    <div class="descriptor">
      <input tabindex="-1" ref="category" class="value category border" 
        v-model="itask.category" v-on:input="dirty=true"
        v-on:focus="() => setIsEditing(true)" v-on:blur="() => setIsEditing(false)"/>
      <input tabindex="-1" ref="repeat" class="value scheduleIntervalValue border" type="number" 
        v-model.number="itask.schedule_interval_value" 
        v-on:input="dirty=true"
        v-on:focus="() => setIsEditing(true)" v-on:blur="() => setIsEditing(false)"/>
      <select class="value scheduleIntervalType border" v-model="itask.schedule_interval_type" v-on:input="dirty=true">
        <option v-for="t in scheduleIntervalTypes" v-bind:key="t">{{t}}</option>
      </select>
      <div class="button" v-bind:class="{disable: !correctValues(), dirty}" v-on:click="updateTask">{{updateText()}}</div>
      <div class="button" v-on:click="deleteTask" v-if="itask.id !== 0">Delete</div>
    </div>
  </div>
</div>
</template>

<script lang="ts">
import {Component, Prop, Vue, Watch} from 'vue-property-decorator';
import {Task, ScheduleIntervalTypes} from '../models/Task';
import {DateTime} from 'luxon';

@Component
export default class TaskComponent extends Vue {
  @Prop() private task!: Task;
  @Prop() private selected!: boolean;
  @Prop() private setIsEditing!: (value: boolean) => void;
  @Prop() private select!: () => void;
  @Prop() private event!: any[];
  private itask = {...this.task} as Task;
  private edit = false;
  private scheduleIntervalTypes = ScheduleIntervalTypes;
  private displayTimeFormat = 'yyyy-MM-dd HH:mm';
  private scheduleTime = DateTime
    .fromISO(this.itask.schedule_time)
    .toFormat(this.displayTimeFormat);
  private detail = false;
  private dirty = false;
  private showNote = (this.itask.note.length > 0);

  @Watch('task')
  private onTaskChange(task: Task, old: Task) {
    this.itask = {...this.task} as Task;
  }

  @Watch('selected')
  private onSelectedChange(event: boolean, old: boolean) {
    if (event === true) {
      (this.$refs.container as any).focus();
    }
  }

  @Watch('event')
  private onEvent(event: any[], old: any[]) {
    if (this.selected === false || old[1] === event[1] || event[0] === 'none') {
      return;
    }
    if (event[0] === 'complete') {
      this.complete();
    } else if (event[0] === 'expand') {
      this.detail = !this.detail;
    } else if (event[0] === 'text') {
      (this.$refs.text as any).focus();
    } else if (event[0] === 'note') {
      (this.$refs.note as any).focus();
      this.showNote = true;
      this.detail = true;
      this.textAreaResize();
    } else if (event[0] === 'stime') {
      (this.$refs.stime as any).focus();
    } else if (event[0] === 'update') {
      this.updateTask();
    } else if (event[0] === 'escape') {
      (this.$refs.container as any).focus();
      this.setIsEditing(false);
    } else if (event[0] === 'delete') {
      this.deleteTask();
    } else if (event[0] === 'repeat') {
      (this.$refs.repeat as any).focus();
      this.detail = true;
    } else if (event[0] === 'category') {
      (this.$refs.category as any).focus();
      this.detail = true;
    }
  }

  private updateText(): string {
    if (this.itask.id === 0) { return 'Create'; } else { return 'Update'; }
  }

  private containerClass() {
    return {
      past: this.isPast(),
      yesterday: this.isYesterday(),
      present: this.isPresent(),
      tomorrow: this.isTomorrow(),
      future: this.isFuture(),
      completed: this.itask.completed === true,
      selected: this.selected,
    };
  }

  private updateTask() {
    this.itask.schedule_time = DateTime.fromFormat(
      this.scheduleTime, this.displayTimeFormat).toISO();
    this.$emit('update', this.itask);
    this.dirty = false;
  }

  private deleteTask() {
    this.$emit('delete', this.task.id);
  }

  private correctValues(): boolean {
    return DateTime.fromFormat(this.scheduleTime, this.displayTimeFormat).isValid;
  }

  private complete() {
    this.dirty = true;
    if (this.itask.completed) {
      this.itask.completed = false;
      this.updateTask();
      return;
    }
    this.itask.schedule_interval_value = +this.itask.schedule_interval_value;
    if (this.itask.schedule_interval_value === 0) {
      this.itask.completed = true;
      this.updateTask();
      return;
    }
    const currentSchedule = DateTime.fromFormat(this.scheduleTime, this.displayTimeFormat);
    switch (this.itask.schedule_interval_type) {
      case 'Day':
        this.itask.schedule_time = currentSchedule.plus({
          day: this.itask.schedule_interval_value,
        }).toISO();
        break;
      case 'Week':
        this.itask.schedule_time = currentSchedule.plus({
          day: 7 * this.itask.schedule_interval_value,
        }).toISO();
        break;
      case 'Month':
        this.itask.schedule_time = currentSchedule.plus({
          month: this.itask.schedule_interval_value,
        }).toISO();
        break;
      case 'Year':
        this.itask.schedule_time = currentSchedule.plus({
          year: this.itask.schedule_interval_value,
        }).toISO();
        break;
    }
    this.scheduleTime = DateTime
      .fromISO(this.itask.schedule_time)
      .toFormat(this.displayTimeFormat);
    this.updateTask();
    return;
  }

  private textAreaResize() {
    const element: any = this.$refs.note;
    element.style.height = 'auto';
    element.style.height = element.scrollHeight + 5 + 'px';
  }

  private presentDiff(): number {
    const time = DateTime.fromFormat(this.scheduleTime, this.displayTimeFormat);
    const startDay = DateTime.local().startOf('day');
    const endDay = DateTime.local().endOf('day');
    const yesterday = DateTime.local().startOf('day').minus({days: 1});
    const tomorrow = DateTime.local().endOf('day').plus({days: 1});
    if (time < yesterday) {
      return -2;
    } else if (time < startDay) {
      return -1;
    } else if (time < endDay) {
      return 0;
    } else if (time < tomorrow) {
      return 1;
    } else {
      return 2;
    }
  }

  private isPast(): boolean { return this.presentDiff() === -2; }
  private isYesterday(): boolean { return this.presentDiff() === -1; }
  private isPresent(): boolean { return this.presentDiff() === 0; }
  private isTomorrow(): boolean { return this.presentDiff() === 1; }
  private isFuture(): boolean { return this.presentDiff() === 2; }
}
</script>

<style scoped>
.task {
  background-color: #d7e7f7;
  margin: 10px 0 0 0;
  padding: 0.75em 20px;
  width: 100%;
}

.selected {
  background-color: #eaeaea !important;
}

.past {
  box-shadow: 2px 2px 4px red;
}

.yesterday {
  box-shadow: 2px 2px 4px lightcoral;
}

.present {
  box-shadow: 2px 2px 4px blue;
}

.tomorrow {
  box-shadow: 2px 2px 4px lightgreen;
}

.future {
  box-shadow: 2px 2px 4px green;
}

.completed {
  box-shadow: none;
  background-color: #f4fff4;
}

.text {
  font-size: 1.5em;
  width: 100%;
  word-break: break-all;
  outline: none;
  border: none;
  background: none;
}

.showDetail {
  background: url("../assets/round-expand_more-24px.svg");
  background-repeat: no-repeat;
  background-position: center;
  height: 2em;
  width: 5em;
  margin: 0 1em 0 0;
  border: none;
  cursor: pointer;
  box-shadow: 1px 1px 1px 1px grey;
}

.hideDetail {
  background: url("../assets/round-expand_less-24px.svg");
  background-repeat: no-repeat;
  background-position: center;
  height: 2em;
  width: 5em;
  margin: 0 1em 0 0;
  border: none;
  cursor: pointer;
  box-shadow: 1px 1px 1px 1px grey;
}

.complete {
  background: url("../assets/baseline-done-24px.svg");
  background-repeat: no-repeat;
  background-position: center;
  height: 2em;
  width: 5em;
  margin: 0 1em 0 0;
  border: none;
  cursor: pointer;
  box-shadow: 1px 1px 1px 1px grey;
}

.scheduleTime {
  background: #fffded;
  border: none;
  font-weight: bold;
  box-shadow: 1px 1px 1px 1px grey;
  padding: 5px;
  height: 2.2em;
  width: 10em;
  text-align: center;
}

.note {
  background-color: ghostwhite;
  width: 100%;
  max-width: 100%;
  padding: 10px;
  margin: 15px 0 0;
  height: auto;
}

.descriptor {
  margin: 15px 0 0 0;
  display: flex;
  flex-flow: row;
  align-items: flex-end;
}

.value {
  background-color: ghostwhite;
  width: fit-content;
  margin-right: 1em;
  padding: 0.5em;
  font-weight: bold;
  font-family: Arial;
  font-size: 0.85em;
  height: 2.5em;
}

.button {
  margin-right: 1em;
  background-color: ghostwhite;
}

.category {
  width: 8em;
  text-align: center;
}

.scheduleIntervalValue {
  width: 5em;
}

.scheduleIntervalType {
  width: 6em;
}

button {
  margin: 5px 10px 0 0;
  min-width: 5em;
}

.dirty {
  background-color: #bff4c8;
}

.is {
  background-color: #fff6f4;
}
</style>