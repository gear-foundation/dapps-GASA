import { cx } from 'utils';
import styles from './MaximumPlayersWarning.module.scss';

function MaximumPlayersWarning() {
  return (
    <div className={cx(styles.container)}>
      <span className={cx(styles.text, styles.textRed)}>Maximum number of players reached</span>
      <span className={cx(styles.text)}>Please try again later or choose another contract address.</span>
    </div>
  );
}

export { MaximumPlayersWarning };
