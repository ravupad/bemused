<template>
  <router-view/>
</template>

<style>
* {
  box-sizing: border-box;
}

.border {
  border: 1px solid grey;
  box-shadow: 0 0 5px 1px grey;
  border-radius: 4px;
}

.button {
  border: 1px solid grey;
  box-shadow: 0 0 5px 1px grey;
  border-radius: 4px;
  background-color: rgb(201, 238, 146);
  font-weight: bold;
  font-family: Arial;
  font-size: 0.9em;
  color: rgb(27, 15, 2);
  cursor: pointer;
  padding: 0.5em;
  min-width: 5em;
  text-align: center;
  text-decoration: none;
}

.flex-row {
  display: flex;
  flex-direction: row;
  align-items: center; /* things will be aligned center vertically */
  justify-content: space-between; /* even spaced between items in row */
}

.flex-column {
  display: flex;
  flex-direction: column;
  align-items: center;
}
</style>
