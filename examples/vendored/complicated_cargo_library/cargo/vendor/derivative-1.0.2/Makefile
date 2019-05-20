publish_doc:
	./node_modules/gitbook-cli/bin/gitbook.js build
	rm -fr /tmp/_book
	git clone --branch gh-pages --single-branch git@github.com:mcarton/rust-derivative.git /tmp/_book
	rm -rf /tmp/_book/*
	mv _book/* /tmp/_book
	cd /tmp/_book; git add .; git com -am Autogenerate; git push -f
