import {Observable} from 'rxjs';
import {React} from '@raviupadhyay/reactrx';
import s from './css/error.scss';

function blockingError(message) {
  return new Observable(subscriber => {
    blockingErrorCb(message, () => subscriber.complete());
  });
}

function blockingErrorPromise(message) {
  return new Promise((resolve, reject) => {
    blockingErrorCb(message, () => resolve());
  });
}

function blockingErrorCb(message, onclose) {
  function close() {
    element.remove();
    onclose();
  }  
  const element = (
    <div class={s.container}>
      <div class={s.innercontainer}>
        <div class={s.message}>{message}</div>
        <button class={s.closebutton} onclick={close}>
          Close
        </button>
      </div>
    </div>
  );
  document.body.append(element);
  document.activeElement.blur();
}

function getMessageFromException(ex) {
  if (ex.message != null && ex.message.length != 0) return ex.message;
  switch(ex.error_code) {
  case "UsernameAlreadyExists":
    return "Username is not available";
  case "PasswordDoesNotMatch":
    return "Password does not match";
  default:
    return `Unknown Error with ErrorCode: ${ex.error_code}`;
  }
}

export {
  blockingError,
  blockingErrorPromise,
  getMessageFromException
};
