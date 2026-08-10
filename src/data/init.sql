CREATE TABLE IF NOT EXISTS accounts (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    email_address VARCHAR(320) NOT NULL,
    provider VARCHAR(64) NOT NULL,
    username VARCHAR(320) NOT NULL,
    imap_server VARCHAR(255) NOT NULL,
    imap_port SMALLINT UNSIGNED NOT NULL DEFAULT 993,
    smtp_server VARCHAR(255) NOT NULL,
    smtp_port SMALLINT UNSIGNED NOT NULL DEFAULT 587,
    auth_type ENUM('password', 'oauth2') NOT NULL DEFAULT 'password',
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
    PRIMARY KEY (id),
    UNIQUE KEY uq_accounts_email (email_address)
);


CREATE TABLE IF NOT EXISTS folders (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    account_id BIGINT UNSIGNED NOT NULL,
    name VARCHAR(255) NOT NULL,
    remote_id VARCHAR(512) NOT NULL,
    folder_type ENUM(
        'inbox',
        'sent',
        'drafts',
        'trash',
        'spam',
        'archive',
        'custom'
    ) NOT NULL DEFAULT 'custom',
    uid_validity BIGINT UNSIGNED NULL,
    uid_next BIGINT UNSIGNED NULL,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (id),
    UNIQUE KEY uq_folder_remote (account_id,remote_id),
    KEY idx_folders_account (account_id),
    CONSTRAINT fk_folders_account FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS threads (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    account_id BIGINT UNSIGNED NOT NULL,
    subject VARCHAR(998) NOT NULL,
    last_message_at DATETIME(6) NULL,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
    PRIMARY KEY (id),
    KEY idx_threads_account (account_id),
    KEY idx_threads_last_message (account_id,last_message_at),
    CONSTRAINT fk_threads_account FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS emails (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    account_id BIGINT UNSIGNED NOT NULL,
    folder_id BIGINT UNSIGNED NOT NULL,
    thread_id BIGINT UNSIGNED NULL,
    message_id VARCHAR(998) NOT NULL,
    sender VARCHAR(2048) NOT NULL,
    subject VARCHAR(998) NULL,
    body_text LONGTEXT NULL,
    body_html LONGTEXT NULL,
    received_at DATETIME(6) NULL,
    sent_at DATETIME(6) NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    is_starred BOOLEAN NOT NULL DEFAULT FALSE,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    is_draft BOOLEAN NOT NULL DEFAULT FALSE,
    is_replied BOOLEAN NOT NULL DEFAULT FALSE,
    is_forwarded BOOLEAN NOT NULL DEFAULT FALSE,
    -- IMAP synchronization
    remote_uid BIGINT UNSIGNED NULL,
    -- Threading
    in_reply_to VARCHAR(998) NULL,
    references_header TEXT NULL,
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    updated_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6) ON UPDATE CURRENT_TIMESTAMP(6),
    PRIMARY KEY (id),
    UNIQUE KEY uq_email_account_message (account_id,message_id),
    UNIQUE KEY uq_email_folder_uid (folder_id,remote_uid),
    KEY idx_emails_account (account_id),
    KEY idx_emails_folder (folder_id),
    KEY idx_emails_thread (thread_id),
    KEY idx_emails_received (account_id,received_at),
    KEY idx_emails_sender (sender(255)),
    KEY idx_emails_unread (account_id,is_read,received_at),
    CONSTRAINT fk_emails_account FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE,
    CONSTRAINT fk_emails_folder FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE CASCADE,
    CONSTRAINT fk_emails_thread FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS attachments (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    email_id BIGINT UNSIGNED NOT NULL,
    filename VARCHAR(255) NOT NULL,
    mime_type VARCHAR(255) NOT NULL,
    size BIGINT UNSIGNED NOT NULL DEFAULT 0,
    sha256 BINARY(32) NULL,
    storage_key VARCHAR(1024) NOT NULL,
    content_id VARCHAR(998) NULL,
    disposition ENUM('inline','attachment') NOT NULL DEFAULT 'attachment',
    created_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6),
    PRIMARY KEY (id),
    KEY idx_attachments_email (email_id),
    KEY idx_attachments_sha256 (sha256),
    CONSTRAINT fk_attachments_email FOREIGN KEY (email_id) REFERENCES emails(id) ON DELETE CASCADE
);