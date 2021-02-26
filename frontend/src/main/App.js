// import logo from './logo.svg';
import './App.css';
import { Input, Button, Row, Col, Layout } from 'antd';
import { useState } from 'react';
import axios from 'axios';
import 'antd/dist/antd.css';
import UserCard from './components/UserCard';

const { Header, Footer, Sider, Content } = Layout;

function App() {
	const style = { background: 'rgb(150,250,180)', padding: '100px 0px', borderRadius: '20px' };
	const blank_box = { background: '', padding: '80px 0px' };

	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');

	const onSubmit = async () => {
		if (username === '') {
			alert('Please enter your username');
		} else if (password === '') {
			alert('Please enter your password');
		} else {
			try {
				let data = await axios.post('http://localhost:5000/users', {
					username: username,
					passwd: password,
				});
				console.log(data);
			} catch (e) {
				console.log(e);
			}
		}

		// send the username and password to the backend to get verified
		// let authKey = await getAuthKey();
		// console.log(authKey);
	};

	const getAuthKey = async () => {
		let data = await axios.post('http://localhost:5000/users',
			// {params: {
			// 	'username': username,
			// 	'passwd': password,
			// }}
			// {
			// 	'username': username,
			// 	'passwd': password,
			// }
		);

		return data;
	};

	const onInput = (fn) => (e) => {
		fn(e.target.value);
	}

	return (
		<div className="root">
			{/* <Row>
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
			</Row> */}
			
			{/* <div style={{ padding:'50vh 5vw' }}>
				<Row gutter={[16, 48]}>
					<Col className="gutter-row" span={6}>
						<div style={style}>bruh</div>
					</Col>
					<Col className="gutter-row" span={6}>
						<div style={blank_box}/>
					</Col>
					<Col className="gutter-row" span={6}>
						<div style={blank_box}/>
					</Col>
					<Col className="gutter-row" span={6}>
						<div style={blank_box}/>
					</Col>
					<Col className="gutter-row" span={6}>
						<div style={style}>col-6</div>
					</Col>
				</Row>
			</div> */}
			{/* <Divider orientation="left">Horizontal</Divider> */}

			{/* <Layout>
				<Sider style={{ background:'rgb(150,250,180)' }}>
					<div style={{ width:'100vw' }}>asdf</div>
				</Sider>
				<Content>
					<div style={{ height:'100vh' }}>
						<h1>bruh</h1>
					</div>
				</Content>
			</Layout> */}
			<div className='half'>
				<UserCard setUsername={onInput(setUsername)} setPasswd={onInput(setPassword)} usernameText='username' passwdText='password' onClick={onSubmit} />
			</div>
		</div>
	);
}

export default App;
