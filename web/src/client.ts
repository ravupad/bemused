import axios, {AxiosPromise, AxiosRequestConfig} from 'axios';
import router from '@/router';
import {Task} from '@/models/Task';
import Article, {ListViewArticle} from '@/models/Article';

const client = axios.create({
  baseURL: '/api',
});

let sessionId = localStorage.getItem('SessionId');

function config(): AxiosRequestConfig {
  return {
    headers: {
      RequestId: Math.random().toString(36).substr(2, 9),
      Authorization: sessionId,
    },
  };
}

function get<T>(url: string): AxiosPromise<T> {
  return client.get(url, config());
}

function post<T>(url: string, body: any): AxiosPromise<T> {
  return client.post(url, body, config());
}

function put<T>(url: string, body: any): AxiosPromise<T> {
  return client.put(url, body, config());
}

function del<T>(url: string): AxiosPromise<T> {
  return client.delete(url, config());
}

client.interceptors.response.use(
    (response) => response,
    (error) => {
      if (error.response.status === 401) {
        localStorage.setItem('SessionId', 'None');
        router.push('/login');
      }
      return Promise.reject(error);
    });

export function signup(username: string, password: string): AxiosPromise<any> {
  return post(`/user/${username}/${password}`, null);
}

export function login(username: string, password: string): Promise<any> {
  return put<string>(`/user/${username}/${password}`, null)
      .then((res) => {
        sessionId = res.data;
        localStorage.setItem('SessionId', res.data);
      });
}

export function verifySessionWithoutRedirect(): AxiosPromise<any> {
  return axios.get(`/api/user/${sessionId}`, config());
}

export function verifySession(): AxiosPromise<any> {
  return get(`/user/${sessionId}`);
}

export function clearSession(): Promise<any> {
  localStorage.setItem('SessionId', '');
  sessionId = null;
  return del(`/user/${sessionId}`);
}

export function getTasks(): AxiosPromise<any> {
  return get(`/task`);
}

export function createTask(task: any): AxiosPromise<any> {
  task.id = 0;
  task.user_id = 0;
  task.completed = false;
  return post(`/task`, task);
}

export function updateTask(task: Task): AxiosPromise<any> {
  return put(`/task`, task);
}

export function deleteTask(taskId: number): AxiosPromise<any> {
  return del(`/task/${taskId}`);
}

export function createArticle(article: Article): AxiosPromise<any> {
  return post('/article', article);
}

export function updateArticle(article: Article): AxiosPromise<any> {
  return put('/article', article);
}

export function getArticle(id: string): AxiosPromise<Article> {
  return get(`/article/${id}`);
}

export function getArticles(): AxiosPromise<ListViewArticle[]> {
  return get('/article');
}

export function deleteArticle(id: string): AxiosPromise<any> {
  return del(`/article/${id}`);
}
