import { useState } from 'react';
import { Input, Button, Row, Col, Layout, Space } from 'antd';

const UserCard = ({ onClick, usernameText, passwdText, setUsername, setPasswd }) => {
    const [text, setText] = useState('');
    const [day, setDay] = useState('');
    const [reminder, setReminder] = useState(false);

    // const onSubmitPreparation = (e) => {
    //     e.preventDefault();
    //     if (text && day) {
    //         onSubmit[0](text, day, reminder);

    //         setText('');
    //         setDay('');
    //         setReminder(false);
    //     }
    //     else alert('Please fill in both text fields');

    //     onSubmit[1](false);
    // };

    return (
        // <form className='add-form' onSubmit={onSubmitPreparation} >
        //     <AddTaskField
        //         title='Task'
        //         inputType='text'
        //         placeHolder='Add Task'
        //         value={text}
        //         onChange={(e) => setText(e.target.value)}
        //     />
        //     <AddTaskField
        //         title='Day and Time'
        //         inputType='text'
        //         placeHolder='Add Day and Time'
        //         value={day}
        //         onChange={(e) => setDay(e.target.value)}
        //     />
        //     <AddTaskField
        //         title='Set Reminder'
        //         inputType='checkbox'
        //         className='form-control-check'
        //         value={reminder}
        //         onChange={(e) => setReminder(e.currentTarget.checked)}
        //     />

        //     <input type='submit' value='Save Task' className='btn btn-block'/>
        // </form>
        <div className='container'>
            <Space direction='vertical' >
                <Input placeholder={usernameText} onChange={setUsername} />
                <Input placeholder={passwdText} onChange={setPasswd} />
                <Button onClick={onClick}>Login</Button>
            </Space>
        </div>
    )
};

export default UserCard;
