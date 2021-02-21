// import logo from './logo.svg';
import './App.css';
import { Input, Button, Row, Col } from 'antd';
import { useState } from 'react';
import axios from 'axios';
import 'antd/dist/antd.css';

function App() {
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');

	const onSubmit = async () => {
		if (username === '') {
			alert('Please enter your username');
		}
		else if (password === '') {
			alert('Please enter your password');
		}

		// send the username and password to the backend to get verified
		let authKey = await getAuthKey();
		console.log(authKey);
	};

	const getAuthKey = async () => {
		let data = await axios.get('http://localhost:5000/', {
			params: {
				username: username,
				password: password,
			}
		});

		return data;
	};

	const inputChange = (setFunc) => (e) => {
		setFunc(e.target.value);
	}

	return (
		<div className="App" >
			<Row>
				<Col span={24}>
					<Input placeholder="username" onChange={inputChange(setUsername)} />
				</Col>
			</Row>
			<Row>
				<Col span={24}>
					<Input placeholder="password" onChange={inputChange(setPassword)} />
				</Col>
			</Row>
			<Row>
				<Col span={24}>
					<Button type="primary" onClick={onSubmit} >Submit</Button>
				</Col>
			</Row>
		</div>
	);
}

export default App;
