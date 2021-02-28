import {route} from "../index";

const baseURL = '/api';
let sessionId = localStorage.getItem('SessionId');
export let userId = localStorage.getItem("UserId");

const headers = () => {
  return {
    RequestId: Math.random().toString(36),
    SessionId: sessionId,
  };
};

const clearCredential = () => {
  localStorage.removeItem('SessionId');
  localStorage.removeItem('UserId');
  userId = null;
  sessionId = null;
};

const fetch = (url, options) =>
  window.fetch(`${baseURL}${url}`, {
    headers: headers(),
    ...options
  }).catch(e => Promise.reject({
    error_code: "FETCH_ERROR",
    message: e
  })).then((response) => {
    if (!response.ok) {
      if (response.status === 401) {
        clearCredential();
        if (window.location.pathname !== '/login') {
          route.next("/login");
        }
        return Promise.reject("You are not logged in.");
      } else {
        return response.json().catch(() => ({
          message: "Not able to reach server. Please try again after some time."
        })).then(json => Promise.reject(json));
      }
    } else if (response.status === 204) {
      return undefined;
    } else if (response.headers.get('content-length') == '0') {
      return undefined;
    } else if (response.headers.get('content-type') == 'application/json') {
      return response.json();
    } else {
      return response.blob();
    }
  });

export function get(url) {
  return fetch(url, {method: 'GET'});
}

export function put(url, body) {
  const options = { method: "PUT" };
  if (body != undefined) options.body = JSON.stringify(body);
  return fetch(url, options);
}

export function post(url, body) {
  const options = {method: "POST"};
  if (body != undefined) options.body = JSON.stringify(body);
  return fetch(url, options);
}

export function del(url) {
  return fetch(url, {
    method: 'DELETE'
  });
}

export function patch(url, body) {
  const options = {method: "PATCH"};
  if (body != undefined) options.body = JSON.stringify(body);
  return fetch(url, options);
}

export function verifySession() {
  if (sessionId == null) return Promise.reject("SessionId is null");
  return get(`/user/info`).then(res => {
    userId = res;
    localStorage.setItem('UserId', userId);
  });
}

export function logout() {
  let temp = sessionId;
  clearCredential();
  return del(`/user/${temp}`);
};

export function checkUsernameAvailability (username) {
  return get(`/user/available/${username}`);
}

export function signup(username, password) {
  return put(`/user/${username}/${password}`);
}

export function login (username, password) {
  return post(`/user/${username}/${password}`).then(res => {
    sessionId = res;
    localStorage.setItem('SessionId', res);
  });
}
