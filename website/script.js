document.querySelectorAll('.copy-btn').forEach(function(btn) {
    btn.addEventListener('click', function() {
        var text = this.getAttribute('data-copy');
        navigator.clipboard.writeText(text).then(function() {
            var orig = this.textContent;
            this.textContent = 'Copiado';
            this.classList.add('copied');
            var self = this;
            setTimeout(function() {
                self.textContent = orig;
                self.classList.remove('copied');
            }, 2000);
        }.bind(this));
    });
});
