import { mount } from 'svelte';
import App from './App.svelte';
import './kieu.css';

const goc = document.getElementById('app');
if (!goc) throw new Error('không tìm thấy #app trong index.html');

export default mount(App, { target: goc });
